use core::fmt;

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    message: String,
}

impl Response {
    fn new(status_code: StatusCode, msg: String) -> Response {
        Response {
            status: status_code,
            message: msg,
        }
    }
    pub fn get_status(&self) -> StatusCode {
        self.status
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[response status: {} message: {}]",
            self.status, self.message
        )
    }
}

pub fn get_ok_response(msg: String) -> Response {
    Response::new(StatusCode::OK, msg)
}

pub fn get_err_response(msg: String) -> Response {
    Response::new(StatusCode::ERR, msg)
}
