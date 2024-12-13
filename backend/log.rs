use std::{borrow::Cow, fmt::Display};

pub fn log<'a, S>(s: S)
where
    S: Into<Log<'a>>,
{
    println!("{}", s.into());
}

#[derive(Debug)]
pub struct Log<'a> {
    msg: Cow<'a, str>,
}

impl<'a> Log<'a> {
    pub fn new(msg: Cow<'a, str>) -> Self {
        Log { msg }
    }
    pub fn as_msg(self) -> Cow<'a, str> {
        self.msg
    }
}

impl<'a> Display for Log<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.msg)
    }
}

impl<'a> From<&'a str> for Log<'a> {
    fn from(msg: &'a str) -> Self {
        Log::new(Cow::Borrowed(msg))
    }
}

impl<'a> From<String> for Log<'a> {
    fn from(msg: String) -> Self {
        Log::new(Cow::Owned(msg))
    }
}

impl<'a> From<&super::ValidatedBERequest<'a>> for Log<'a> {
    fn from(req: &super::ValidatedBERequest) -> Self {
        Log::new(Cow::Owned(format!(
            "[INFO] Received valid request: {} {}\n{}",
            req.req.method(),
            req.req.url(),
            match &req.task {
                crate::TaskData::GptQuery { query } => query,
            }
        )))
    }
}

impl<'a> From<&super::BEReqError<'a>> for Log<'a> {
    fn from(err: &super::BEReqError) -> Self {
        use super::ErrorType as E;
        Log::new(Cow::Owned(format!(
            "[WARNING] Recieved invalid request: {} {}\n{}",
            err.req.method(),
            err.req.url(),
            match &err.error {
                E::UrlParseError { err } => format!("Unable to parse URL: {err}"),
                E::UrlInvalidPath { path } => format!("Provided path is invalid: {path}"),
                E::UrlInvalidQuery { query, expected } => format!(
                    "Expected following query fields: {:?}\nGot: {}",
                    expected,
                    query.clone().unwrap_or("nothing".to_string())
                ),
                E::HttpIncorrectMethod { got, expected } =>
                    format!("Incorrect http method | Got: {got} | Expected: {expected}"),
                E::UrlencodedDecodeError { query_value, err } =>
                    format!("Error while decoding urlencoded value: {query_value} | {err}"),
            }
        )))
    }
}
