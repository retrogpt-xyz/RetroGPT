use std::{
    borrow::Cow,
    fs::OpenOptions,
    io::{self, stderr, Stderr, Stdout, Write},
    path::PathBuf,
};

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    cfg: LogCfg,
}

impl Logger {
    pub fn from_cfg(cfg: LogCfg) -> Self {
        Self { cfg }
    }
}

impl Logger {
    pub fn stderr() -> Self {
        Self {
            cfg: LogCfg {
                debug: vec![Box::new(stderr())],
                info: vec![Box::new(stderr())],
                warning: vec![Box::new(stderr())],
                error: vec![Box::new(stderr())],
            },
        }
    }

    pub fn debug<T: LogFmt>(&self, item: T) {
        item.fmt_debug()
            .map(|msg| {
                self.cfg.debug.iter().for_each(|dest| {
                    let _ = dest.write_log(msg.as_ref());
                })
            })
            .unwrap_or(())
    }
    pub fn info<T: LogFmt>(&self, item: T) {
        item.fmt_info()
            .map(|msg| {
                self.cfg.info.iter().for_each(|dest| {
                    let _ = dest.write_log(msg.as_ref());
                })
            })
            .unwrap_or(())
    }
    pub fn warning<T: LogFmt>(&self, item: T) {
        item.fmt_warning()
            .map(|msg| {
                self.cfg.warning.iter().for_each(|dest| {
                    let _ = dest.write_log(msg.as_ref());
                })
            })
            .unwrap_or(())
    }
    pub fn error<T: LogFmt>(&self, item: T) {
        item.fmt_error()
            .map(|msg| {
                self.cfg.error.iter().for_each(|dest| {
                    let _ = dest.write_log(msg.as_ref());
                })
            })
            .unwrap_or(())
    }
    pub fn log_at<T: LogFmt>(&self, item: T, level: LogLevel) {
        match level {
            LogLevel::Debug => self.debug(item),
            LogLevel::Info => self.info(item),
            LogLevel::Warning => self.warning(item),
            LogLevel::Error => self.error(item),
        }
    }
}

pub struct LogCfg {
    debug: Vec<Box<dyn LogDest>>,
    info: Vec<Box<dyn LogDest>>,
    warning: Vec<Box<dyn LogDest>>,
    error: Vec<Box<dyn LogDest>>,
}

impl LogCfg {
    pub fn new(
        debug: Vec<Box<dyn LogDest>>,
        info: Vec<Box<dyn LogDest>>,
        warning: Vec<Box<dyn LogDest>>,
        error: Vec<Box<dyn LogDest>>,
    ) -> Self {
        Self {
            debug,
            info,
            warning,
            error,
        }
    }
}

pub trait LogFmt {
    fn fmt_debug<'a>(&'a self) -> io::Result<Cow<'a, str>>;
    fn fmt_info<'a>(&'a self) -> io::Result<Cow<'a, str>>;
    fn fmt_warning<'a>(&'a self) -> io::Result<Cow<'a, str>>;
    fn fmt_error<'a>(&'a self) -> io::Result<Cow<'a, str>>;
}

impl LogFmt for &str {
    fn fmt_debug<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Borrowed(self))
    }

    fn fmt_info<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Borrowed(self))
    }

    fn fmt_warning<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Borrowed(self))
    }

    fn fmt_error<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Borrowed(self))
    }
}

impl LogFmt for io::Error {
    fn fmt_debug<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_info<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_warning<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_error<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }
}

impl LogFmt for toml::de::Error {
    fn fmt_debug<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_info<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_warning<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }

    fn fmt_error<'a>(&'a self) -> io::Result<Cow<'a, str>> {
        Ok(Cow::Owned(format!("{}", self)))
    }
}

pub trait LogDest {
    fn write_log(&self, log: &str) -> io::Result<()>;
}

impl LogDest for Stdout {
    fn write_log(&self, log: &str) -> io::Result<()> {
        writeln!(self.lock(), "{}", log)
    }
}

impl LogDest for Stderr {
    fn write_log(&self, log: &str) -> io::Result<()> {
        writeln!(self.lock(), "{}", log)
    }
}

impl LogDest for PathBuf {
    fn write_log(&self, log: &str) -> io::Result<()> {
        writeln!(
            OpenOptions::new().create(true).append(true).open(self)?,
            "{}",
            log
        )
    }
}
