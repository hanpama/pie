pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        Self { msg: msg.into() }
    }
}

pub type AnyError = Box<dyn std::error::Error>;
