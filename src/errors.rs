use std::fmt::{Display, Formatter};
use std::sync::Arc;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, web};
use mime_guess::mime;
use minijinja::Environment;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ViewError<'a> {
    pub code: u16,
    pub kind: u8,
    #[serde(skip)]
    pub jinja: Arc<Environment<'a>>
}

impl ViewError<'_> {
    pub fn new(jinja: Arc<Environment>, code: StatusCode, kind: u8) -> Self {
        ViewError {
            code: code.as_u16(),
            kind,
            jinja,
        }
    }
}

impl Display for ViewError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ViewError: code: {}, kind: {}", self.code, self.kind)
    }
}

impl ResponseError for ViewError<'_> {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(web::Json(self))
    }
}

