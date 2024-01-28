mod diff;
mod embed;
mod format;
mod hash;
mod routes;
mod state;
mod utils;

use crate::embed::{get_embed_static_file, get_embed_template_file};
use crate::routes::{get_diff, get_format, get_hash, post_diff, post_format, post_hash, statics};
use crate::state::AppState;
use actix_web::web;
use actix_web::{App, HttpServer};
use log::info;
use minijinja::{Environment};
use quickjs_rs::Context;
use std::path::Path;
use std::sync::Arc;

fn init_js_rt() -> Context {
    let js_beautify = get_embed_static_file("js-beautify.js");
    let diff2html = get_embed_static_file("diff2html.js");
    let source = format!(
        "{}\n{}",
        String::from_utf8_lossy(&js_beautify),
        String::from_utf8_lossy(&diff2html)
    );

    let context = Context::new().unwrap();
    context.eval(&source).unwrap();
    info!("init js runtime: {}bytes", source.len());
    context
}

#[allow(unused_variables)]
fn init_template<'source>(dir: &Path) -> Environment<'source> {
    let mut env: Environment = Environment::new();

    env.set_loader(|name| {
        // info!("load template: {}", name);
        let result = get_embed_template_file(name);
        Ok(Some(String::from_utf8_lossy(&result).into()))
    });

    return env;
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".to_string());
    let app_port = std::env::var("APP_PORT").unwrap_or("8888".to_string()).parse::<u16>().unwrap();

    let jinja_env = Arc::new(init_template(Path::new("src/templates")));

    HttpServer::new(move || {
        let js_rt = init_js_rt();
        let state = web::Data::new(AppState {
            jinja_env: jinja_env.clone(),
            js_rt,
        });

        App::new()
            .app_data(state)
            .app_data(web::FormConfig::default().limit(1024 * 1024 * 10))
            .service(statics)
            .service(get_format)
            .service(post_format)
            .service(get_diff)
            .service(post_diff)
            .service(get_hash)
            .service(post_hash)
    })
    .bind((app_host, app_port))
    .unwrap()
    .run()
    .await
}
