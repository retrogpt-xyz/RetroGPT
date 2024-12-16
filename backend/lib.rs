mod log;

pub use log::log;

use log::Logger;
use serde_json::json;
use std::borrow::Cow;
use tiny_http::{Header, Request, Response, Server, StatusCode};
use url::Url;
use urlencoding::decode;

fn setup_server(cfg: &BEConfig) -> Option<Server> {
    match Server::http(format!("0.0.0.0:{}", cfg.port)) {
        Ok(s) => {
            log("Successfully started server");
            Some(s)
        }
        Err(_) => {
            log("Unable to start server. Aborting");
            None
        }
    }
}

pub fn run_server(cfg: BEConfig) {
    setup_server(&cfg).map(move |server| {
        for req in server.incoming_requests() {
            PrimReqWrapper(req, &cfg)
                .try_into()
                .and_then(validate_request)
                .inspect(|r| {
                    let _ = cfg.logger.info(r);
                })
                .and_then(build_response_ok)
                .inspect_err(|e| log(e))
                .unwrap_or_else(build_response_err)
                .respond();
        }
    });
}

struct PrimReqWrapper<'a>(Request, &'a BEConfig);

struct DBReq<'a> {
    req: Request,
    url: Url,
    task: TaskType,
    cfg: &'a BEConfig,
}

enum TaskType {
    GptQuery,
}

impl<'a> TryFrom<PrimReqWrapper<'a>> for DBReq<'a> {
    type Error = BEReqError<'a>;
    fn try_from(rw: PrimReqWrapper<'a>) -> Result<DBReq<'a>, Self::Error> {
        let req = rw.0;
        let cfg = rw.1;
        let (req, url) = parse_req_url(req, cfg)?;
        let (req, url, task) = parse_req_task(req, url, cfg)?;
        Ok(Self {
            req,
            url,
            task,
            cfg,
        })
    }
}

fn parse_req_url(req: Request, cfg: &BEConfig) -> Result<(Request, Url), BEReqError> {
    match Url::parse(&format!("http://localhost:{}", cfg.port)).and_then(|url| url.join(req.url()))
    {
        Ok(u) => Ok((req, u)),
        Err(e) => Err(BEReqError {
            error: ErrorType::UrlParseError { err: e },
            cfg,
            req,
        }),
    }
}

fn parse_req_task<'a>(
    req: Request,
    url: Url,
    cfg: &'a BEConfig,
) -> Result<(Request, Url, TaskType), BEReqError<'a>> {
    match url.path_segments().into_iter().flatten().next() {
        Some(p) if p == "gptquery" => Ok((req, url, TaskType::GptQuery)),
        _ => Err(BEReqError {
            error: ErrorType::UrlInvalidPath {
                path: url.path().to_string(),
            },
            cfg,
            req,
        }),
    }
}

struct ValidatedBERequest<'a> {
    req: Request,
    task: TaskData,
    cfg: &'a BEConfig,
}

enum TaskData {
    GptQuery { query: String },
}

fn validate_request<'a>(req: DBReq<'a>) -> Result<ValidatedBERequest<'a>, BEReqError<'a>> {
    let (task_data, req) = parse_url_task_data(req)?;
    let req = parse_url_task_method(req)?;
    Ok(ValidatedBERequest {
        req: req.req,
        task: task_data,
        cfg: req.cfg,
    })
}

fn parse_url_task_data(req: DBReq) -> Result<(TaskData, DBReq), BEReqError> {
    match req.task {
        TaskType::GptQuery => match req.url.query_pairs().filter(|(k, _)| k == "query").next() {
            Some((_, v)) => Ok((
                TaskData::GptQuery {
                    query: match decode(&v) {
                        Ok(s) => s.into_owned(),
                        Err(e) => {
                            return Err(BEReqError {
                                error: ErrorType::UrlencodedDecodeError {
                                    query_value: v.into_owned(),
                                    err: e,
                                },
                                req: req.req,
                                cfg: req.cfg,
                            })
                        }
                    },
                },
                req,
            )),
            None => Err(BEReqError {
                error: ErrorType::UrlInvalidQuery {
                    query: req.url.query().map(str::to_string),
                    expected: vec!["query".to_string()],
                },
                cfg: req.cfg,
                req: req.req,
            }),
        },
    }
}

fn parse_url_task_method(req: DBReq) -> Result<DBReq, BEReqError> {
    match req.task {
        TaskType::GptQuery => match req.req.method() {
            tiny_http::Method::Get => Ok(req),
            _ => Err(BEReqError {
                error: ErrorType::HttpIncorrectMethod {
                    got: req.req.method().clone(),
                    expected: tiny_http::Method::Get,
                },
                cfg: req.cfg,
                req: req.req,
            }),
        },
    }
}

pub struct BEReqError<'a> {
    req: Request,
    error: ErrorType,
    cfg: &'a BEConfig,
}

pub enum ErrorType {
    UrlParseError {
        err: url::ParseError,
    },
    UrlInvalidPath {
        path: String,
    },
    UrlInvalidQuery {
        query: Option<String>,
        expected: Vec<String>,
    },
    HttpIncorrectMethod {
        got: tiny_http::Method,
        expected: tiny_http::Method,
    },
    UrlencodedDecodeError {
        query_value: String,
        err: std::string::FromUtf8Error,
    },
}

struct BEResponse<'a> {
    req: Request,
    cfg: &'a BEConfig,
    body: serde_json::Value,
    headers: Vec<Header>,
    status_code: StatusCode,
}

impl<'a> BEResponse<'a> {
    fn respond(self) {
        let mut resp =
            Response::from_string(self.body.to_string()).with_status_code(self.status_code);
        for h in self.headers {
            resp.add_header(h);
        }
        if let Err(_) = self.req.respond(resp) {
            log("Encountered error while responding to request");
        }
    }
}

fn build_response_ok<'a>(r: ValidatedBERequest<'a>) -> Result<BEResponse<'a>, BEReqError<'a>> {
    match r.task {
        TaskData::GptQuery { query } => Ok(BEResponse {
            headers: vec![r.cfg.json_header.clone()],
            req: r.req,
            cfg: r.cfg,
            body: json!({ "gpt_resp": query_gpt(query)? }),
            status_code: StatusCode(200),
        }),
    }
}

fn query_gpt<'a>(s: impl Into<Cow<'a, str>>) -> Result<Cow<'a, str>, BEReqError<'a>> {
    Ok(s.into())
}

fn build_response_err<'a>(e: BEReqError<'a>) -> BEResponse<'a> {
    let (msg, code) = match e.error {
        ErrorType::UrlParseError { .. } => (
            format!("Internal Error: Error occured while parsing url"),
            500,
        ),
        ErrorType::UrlInvalidPath { .. } => (format!("Bad request: URL path was not valid"), 400),
        ErrorType::UrlInvalidQuery { expected, .. } => (
            format!("Bad request: The following query keys are required: {expected:?}"),
            400,
        ),
        ErrorType::HttpIncorrectMethod { expected, .. } => (
            format!("Bad request: Expected following HTTP method: {expected}"),
            0,
        ),
        ErrorType::UrlencodedDecodeError { .. } => (
            format!("Internal Error: Error occured while decoding query"),
            500,
        ),
    };
    let body = json!({ "error": msg });
    BEResponse {
        cfg: e.cfg,
        body,
        req: e.req,
        headers: vec![e.cfg.json_header.clone()],
        status_code: StatusCode(code),
    }
}

pub struct BEConfig {
    port: i32,
    json_header: Header,
    logger: log::Logger,
}

pub fn get_cfg() -> Option<BEConfig> {
    let port = 1985;
    let json_header = Header::from_bytes("Content-Type", "application/json").ok()?;
    let logger = Logger {
        cfg: log::LogCfg {
            debug: vec![Box::new(std::io::stderr())],
            info: vec![Box::new(std::io::stderr())],
            warn: vec![Box::new(std::io::stderr())],
            error: vec![Box::new(std::io::stderr())],
        },
    };
    let _ = logger.info("Got config");
    Some(BEConfig {
        port,
        json_header,
        logger,
    })
}
