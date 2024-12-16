use std::{borrow::Cow, fs::OpenOptions, io::Write, path::Path};

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct Logger {
    pub cfg: LogCfg,
}

impl Logger {
    pub fn debug<'a>(&self, l: impl Into<Log<'a>>) -> Result<(), std::io::Error> {
        let log = l.into();
        self.cfg.debug.iter().map(|d| d.write_log(&log)).collect()
    }

    pub fn info<'a>(&self, l: impl Into<Log<'a>>) -> Result<(), std::io::Error> {
        let log = l.into();
        self.cfg.info.iter().map(|d| d.write_log(&log)).collect()
    }

    pub fn warn<'a>(&self, l: impl Into<Log<'a>>) -> Result<(), std::io::Error> {
        let log = l.into();
        self.cfg.warn.iter().map(|d| d.write_log(&log)).collect()
    }

    pub fn error<'a>(&self, l: impl Into<Log<'a>>) -> Result<(), std::io::Error> {
        let log = l.into();
        self.cfg.error.iter().map(|d| d.write_log(&log)).collect()
    }

    pub fn log<'a>(&self, l: impl Into<Log<'a>>, level: LogLevel) -> Result<(), std::io::Error> {
        match level {
            LogLevel::Debug => self.debug(l),
            LogLevel::Info => self.info(l),
            LogLevel::Warn => self.warn(l),
            LogLevel::Error => self.error(l),
        }
    }
}

pub struct LogCfg {
    pub debug: Vec<Box<dyn LogDest>>,
    pub info: Vec<Box<dyn LogDest>>,
    pub warn: Vec<Box<dyn LogDest>>,
    pub error: Vec<Box<dyn LogDest>>,
}

pub trait LogDest {
    fn write_log<'a>(&self, l: &Log<'a>) -> std::io::Result<()>;
}

impl LogDest for std::io::Stdout {
    fn write_log<'a>(&self, l: &Log<'a>) -> std::io::Result<()> {
        writeln!(self.lock(), "{}", l.as_msg())
    }
}

impl LogDest for std::io::Stderr {
    fn write_log<'a>(&self, l: &Log<'a>) -> std::io::Result<()> {
        writeln!(self.lock(), "{}", l.as_msg())
    }
}

impl LogDest for Path {
    fn write_log<'a>(&self, l: &Log<'a>) -> std::io::Result<()> {
        writeln!(
            OpenOptions::new().create(true).append(true).open(self)?,
            "{}",
            l.as_msg()
        )
    }
}

pub fn log<'a, S>(_: S) {
    println!("\nthis is where a log would be");
}

pub struct Log<'a> {
    msg: Cow<'a, str>,
}

impl<'a> Log<'a> {
    pub fn as_msg(&self) -> &str {
        &self.msg
    }
}

// #[derive(Debug)]
// pub struct Log<'a> {
// msg: Cow<'a, str>,
// }
//
// impl<'a> Log<'a> {
// pub fn new(msg: Cow<'a, str>) -> Self {
// Log { msg }
// }
// pub fn as_msg(self) -> Cow<'a, str> {
// self.msg
// }
// }
//
// impl<'a> Display for Log<'a> {
// fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// write!(f, "{}\n", self.msg)
// }
// }
//
impl<'a> From<&'a str> for Log<'a> {
    fn from(msg: &'a str) -> Self {
        Log {
            msg: Cow::Borrowed(msg),
        }
    }
}
impl<'a> From<String> for Log<'a> {
    fn from(msg: String) -> Self {
        Log {
            msg: Cow::Owned(msg),
        }
    }
}

impl<'a> From<&super::ValidatedBERequest<'a>> for Log<'a> {
    fn from(req: &super::ValidatedBERequest) -> Self {
        Log {
            msg: Cow::Owned(format!(
                "[INFO] Received valid request: {} {}\n{}",
                req.req.method(),
                req.req.url(),
                match &req.task {
                    crate::TaskData::GptQuery { query } => query,
                }
            )),
        }
    }
}

// impl<'a> From<&super::BEReqError<'a>> for Log<'a> {
// fn from(err: &super::BEReqError) -> Self {
// use super::ErrorType as E;
// Log::new(Cow::Owned(format!(
// "[WARNING] Recieved invalid request: {} {}\n{}",
// err.req.method(),
// err.req.url(),
// match &err.error {
// E::UrlParseError { err } => format!("Unable to parse URL: {err}"),
// E::UrlInvalidPath { path } => format!("Provided path is invalid: {path}"),
// E::UrlInvalidQuery { query, expected } => format!(
// "Expected following query fields: {:?}\nGot: {}",
// expected,
// query.clone().unwrap_or("nothing".to_string())
// ),
// E::HttpIncorrectMethod { got, expected } =>
// format!("Incorrect http method | Got: {got} | Expected: {expected}"),
// E::UrlencodedDecodeError { query_value, err } =>
// format!("Error while decoding urlencoded value: {query_value} | {err}"),
// }
// )))
// }
// }
