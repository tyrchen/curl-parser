pub(crate) mod error;
mod parser;

use http::{HeaderMap, Method, Uri};

pub use error::Error;

#[derive(Debug, Clone, Default)]
pub struct ParsedRequest {
    pub method: Method,
    pub url: Uri,
    pub headers: HeaderMap,
    pub body: Vec<String>,
}
