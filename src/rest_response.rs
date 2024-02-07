use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct RestResponse {
    code: u8,
    message: String,
}

#[allow(dead_code)]
impl RestResponse {
    pub fn ok() -> Self {
        RestResponse {
            code: 0,
            message: "".to_string(),
        }
    }
    pub fn err(message: String) -> Self {
        RestResponse {
            code: 1,
            message,
        }
    }
    pub fn query_err(message: String) -> Self {
        RestResponse {
            code: 2,
            message,
        }
    }
}
