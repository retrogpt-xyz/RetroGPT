use tiny_http::{Request, Server};
use url::Url;
use urlencoding::decode;

fn main() -> Result<(), RequestError> {
    let server = Server::http("0.0.0.0:1985").unwrap();
    let incoming_requests = server.incoming_requests();
    for req in incoming_requests {
        let resp = process_request(&req).unwrap_or_else(|e| Response::from_string(format!("Error processing request:\n\n{e:?}")));
        req.respond(resp).unwrap();
    }
    Ok(())
}

type Response = tiny_http::Response<std::io::Cursor<Vec<u8>>>;

fn process_request(r: &Request) -> Result<Response, RequestError> {
    let url = parse_url(&r)?;
    let req = validate_path(&url)?;
    form_response(&req)
}

fn parse_url(r: &Request) -> Result<Url, url::ParseError> {
    Url::parse("http://localhost:1985")?.join(r.url())
}

fn validate_path(u: &Url) -> Result<RetroGPTRequest, RequestError> {
    if u.path() == "/gpt_query" {
        Ok(RetroGPTRequest::GPTQuery(
            decode(
                &u.query_pairs()
                    .filter(|(k, _)| k == "query")
                    .map(|(_, v)| v.to_string())
                    .next()
                    .ok_or(RequestError::InvalidQuery)?,
            )?
            .to_string(),
        ))
    } else {
        Err(RequestError::InvalidPath(u.path().to_string()))
    }
}

fn form_response(req: &RetroGPTRequest) -> Result<Response, RequestError> {
    match req {
        RetroGPTRequest::GPTQuery(q) => Ok(Response::from_string(query_api(q)?)),
    }
}

fn query_api(q: &str) -> Result<String, RequestError> {
    Ok(q.to_string())
}

enum RetroGPTRequest {
    GPTQuery(String),
}

#[derive(Debug)]
pub enum RequestError {
    InvalidPath(String),
    InvalidQuery,
    UrlParseError(url::ParseError),
    QueryParseError(std::string::FromUtf8Error),
    IoError(std::io::Error),
}

impl From<url::ParseError> for RequestError {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParseError(e)
    }
}

impl From<std::string::FromUtf8Error> for RequestError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::QueryParseError(e)
    }
}

impl From<std::io::Error> for RequestError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
