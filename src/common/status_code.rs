use core::fmt;

#[derive(Debug)]
pub enum StatusCode {
    OK = 1,
    ERR = 2,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::OK => "OK",
            Self::ERR => "ERR",
        };
        write!(f, "{}", value)
    }
}
