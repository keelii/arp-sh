use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use minijinja::{Environment, Value};
use quickjs_rs::Context;
use std::sync::Arc;

pub struct AppState {
    pub js_rt: Context,
    pub jinja_env: Arc<Environment<'static>>,
}

impl AppState {
    pub fn render(&self, name: &str, ctx: Value) -> HttpResponse {
        let tmpl = self.jinja_env.get_template(name).unwrap();
        let rv = tmpl.render(ctx).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rv)
    }
    pub fn format_html(&self, text: &str, indent2: &str) -> Result<String, String> {
        let ret = self.js_rt.call_function("format_html", vec![text, indent2]);
        match ret {
            Ok(value) => Ok(value.into_string().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn format_js(&self, text: &str, indent2: &str) -> Result<String, String> {
        let ret = self.js_rt.call_function("format_js", vec![text, indent2]);
        match ret {
            Ok(value) => Ok(value.into_string().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn format_css(&self, text: &str, indent2: &str) -> Result<String, String> {
        let ret = self.js_rt.call_function("format_css", vec![text, indent2]);
        match ret {
            Ok(value) => Ok(value.into_string().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn diff_to_html(&self, diff_type: &str, diff: &str) -> Result<String, String> {
        let ret = self
            .js_rt
            .call_function("diff_to_html", vec![diff_type, diff]);
        match ret {
            Ok(value) => Ok(value.into_string().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }
}
