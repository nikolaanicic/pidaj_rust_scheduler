use core::fmt;

#[derive(Debug)]
pub struct Request {
    pub id: i32,
}

impl Request {
    pub fn new(id: i32) -> Request {
        Request { id: id }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[request id:{}]", self.id)
    }
}
