use core::fmt;

use super::StatusCode;

#[derive(Debug, Clone)]
pub struct Response {
    status: StatusCode,
    message: String,
    conns: i32,
}

impl Response {
    fn new(status_code: StatusCode, msg: String, conns: i32) -> Response {
        Response {
            status: status_code,
            message: msg,
            conns: conns,
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
            "[status: {} message: {} active conns: {}]",
            self.status, self.message, self.conns
        )
    }
}

pub fn get_ok_response(msg: String, conns: i32) -> Response {
    Response::new(StatusCode::OK, msg, conns)
}

pub fn get_err_response(msg: String, conns: i32) -> Response {
    Response::new(StatusCode::ERR, msg, conns)
}
