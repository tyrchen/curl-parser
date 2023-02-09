pub(crate) mod error;
mod parser;

use http::{HeaderMap, Method, Uri};
use std::borrow::Cow;

pub use error::Error;

#[derive(Debug, Clone, Default)]
pub struct ParsedRequest<'a> {
    pub method: Method,
    pub url: Uri,
    pub headers: HeaderMap,
    pub body: Vec<Cow<'a, str>>,
}
