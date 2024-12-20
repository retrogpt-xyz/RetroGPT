use std::{
    fs::File,
    io::{self, read_to_string},
    path::PathBuf,
};
use tap::TapOptional;

use crate::logging::{LogCfg, LogDest, Logger};

pub struct Cfg {
    pub port: u16,
    pub logger: Logger,
}

#[derive(serde::Deserialize, Debug)]
struct DesCfg {
    server: DesServerCfg,
    logging: DesLogCfg,
}

#[derive(serde::Deserialize, Debug)]
struct DesServerCfg {
    port: u16,
}

#[derive(serde::Deserialize, Debug)]
struct DesLogCfg {
    debug: DesLogDest,
    info: DesLogDest,
    warning: DesLogDest,
    error: DesLogDest,
}

#[derive(serde::Deserialize, Debug)]
struct DesLogDest {
    dests: Vec<String>,
}
impl Cfg {
    pub fn get(path: impl Into<PathBuf>) -> Option<Self> {
        let def_lgr = Logger::stderr();
        Self::get_inner(path.into(), &def_lgr)
            .tap_none(|| def_lgr.error("Unable to retrieve config... Aborting"))
    }

    fn get_inner(path: PathBuf, def_lgr: &Logger) -> Option<Self> {
        let conf_file = File::open(path).map_err(|e| def_lgr.error(e)).ok()?;
        let conf_str = read_to_string(conf_file)
            .map_err(|e| def_lgr.error(e))
            .ok()?;
        let conf_toml = toml::from_str::<DesCfg>(&conf_str)
            .map_err(|e| def_lgr.error(e))
            .ok()?;
        let port = conf_toml.server.port;
        let logger = Logger::from_cfg(LogCfg::new(
            Cfg::parse_dests(conf_toml.logging.debug.dests),
            Cfg::parse_dests(conf_toml.logging.info.dests),
            Cfg::parse_dests(conf_toml.logging.warning.dests),
            Cfg::parse_dests(conf_toml.logging.error.dests),
        ));
        Some(Self { port, logger })
    }

    fn parse_dests(dest_str: Vec<String>) -> Vec<Box<dyn LogDest>> {
        dest_str
            .into_iter()
            .map(|s| match s.as_str() {
                "stdout" => Box::new(io::stdout()) as Box<dyn LogDest>,
                "stderr" => Box::new(io::stderr()) as Box<dyn LogDest>,
                path_str => Box::new(PathBuf::from(path_str)) as Box<dyn LogDest>,
            })
            .collect()
    }
}
