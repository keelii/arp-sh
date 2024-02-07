mod diff;
mod embed;
mod format;
mod hash;
mod routes;
mod state;
mod utils;
mod consts;
mod rest_response;

use crate::embed::{get_embed_static_file, get_embed_template_file};
use crate::routes::{get_diff, get_format, get_hash, get_pass, get_uuid, post_diff, post_format, post_hash, statics};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use actix_web::{App, HttpServer};
use log::{info};
use minijinja::{Environment};
use quickjs_rs::Context;
use std::path::Path;
use std::sync::Arc;
use actix_web::error::InternalError;
use crate::rest_response::RestResponse;


fn init_js_rt() -> Context {
    let js_beautify = get_embed_static_file("js-beautify.js");
    let diff2html = get_embed_static_file("diff2html.js");
    let source = format!(
        "{}\n{}",
        String::from_utf8_lossy(&js_beautify),
        String::from_utf8_lossy(&diff2html)
    );

    let context = Context::new().unwrap();

    // let test = "async function test() { Promise.resolve(1); }; await test()";
    // let ret = context.eval(&test).unwrap();
    // println!("=====");
    // println!("test js runtime: {:?}", ret);
    // println!("=====");

    context.eval(&source).unwrap();
    info!("init js runtime: {}bytes", source.len());
    context
}

fn encrypt_filter(value: String) -> String {
    let len = value.chars().count();
    let stars = "*".repeat(len - 4);
    return format!("{}{}{}", &value[0..2], stars, &value[len - 2..])
}
#[allow(unused_variables)]
fn init_template<'source>(dir: &Path) -> Environment<'source> {
    let mut env: Environment = Environment::new();

    env.set_loader(|name| {
        // info!("load template: {}", name);
        let result = get_embed_template_file(name);
        Ok(Some(String::from_utf8_lossy(&result).into()))
    });

    env.add_filter("encrypt", encrypt_filter);

    return env;
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".to_string());
    let app_port = std::env::var("APP_PORT").unwrap_or("8888".to_string()).parse::<u16>().unwrap();

    let jinja_env_global = Arc::new(init_template(Path::new("src/templates")));

    HttpServer::new(move || {
        let jinja_env = jinja_env_global.clone();
        let js_rt = init_js_rt();

        let state = web::Data::new(AppState {
            jinja_env,
            js_rt,
        });

        App::new()
            .app_data(state)
            .app_data(web::FormConfig::default().limit(1024 * 1024 * 10))
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                let msg = err.to_string();
                InternalError::from_response(err, HttpResponse::BadRequest().json(
                    RestResponse::query_err(msg)
                )).into()
            }))
            .service(statics)
            .service(web::redirect("/", "/format"))
            .service(get_format)
            .service(post_format)
            .service(get_diff)
            .service(post_diff)
            .service(get_hash)
            .service(post_hash)
            .service(get_uuid)
            .service(get_pass)
    })
    .bind((app_host, app_port))
    .unwrap()
    .run()
    .await
}

// fn register_handler(jinja: Arc<Environment>) -> fn(QueryPayloadError, &HttpRequest) -> Error {
//     |err: QueryPayloadError, req: &HttpRequest| {
//         ViewError::new(jinja, StatusCode::BAD_GATEWAY, 1).into()
//     }
// }
