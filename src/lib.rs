//! A parser for converting curl commands into structured request objects.
//!
//! This crate provides functionality to parse curl command strings into a structured [`ParsedRequest`]
//! that can be used programmatically. It's particularly useful for:
//!
//! - Converting curl commands into programmatic HTTP requests
//! - Testing and debugging HTTP clients
//! - Generating code from curl examples
//!
//! # Design Goals
//!
//! The main goals of this crate are:
//!
//! - **Accuracy**: Faithfully parse curl commands while preserving all relevant HTTP request details
//! - **Ergonomics**: Provide a simple, intuitive API for parsing and using the results
//! - **Flexibility**: Support common curl options while allowing for future extensibility
//! - **Safety**: Handle malformed input gracefully with meaningful errors
//!
//! # Architecture
//!
//! The crate uses a multi-stage parsing approach:
//!
//! 1. **Lexical Analysis**: Uses Pest parser to tokenize the curl command string according to formal grammar rules
//! 2. **Semantic Analysis**: Converts tokens into structured request components (method, headers, etc.)
//! 3. **Request Building**: Assembles the components into a [`ParsedRequest`] struct
//!
//! The parser handles various curl options including:
//! - HTTP methods (`-X`, `--request`)
//! - Headers (`-H`, `--header`)
//! - Request body (`-d`, `--data`)
//! - Authentication (`-u`)
//! - SSL verification (`-k`, `--insecure`)
//! - URL redirection (`-L`, `--location`)
//!
//! # Examples
//!
//! Basic GET request:
//!
//! ```
//! use curl_parser::ParsedRequest;
//! use std::str::FromStr;
//! # fn main() -> Result<(), curl_parser::Error> {
//! let curl = r#"curl https://api.example.com/users"#;
//! let request = ParsedRequest::from_str(curl)?;
//! # Ok(())
//! # }
//! ```
//!
//! POST request with headers and body:
//!
//! ```
//! use curl_parser::ParsedRequest;
//! use serde_json::json;
//! # fn main() -> Result<(), curl_parser::Error> {
//! let curl = r#"curl -X POST https://api.example.com/users \
//!     -H 'Content-Type: application/json' \
//!     -H 'Authorization: Bearer {{ token }}' \
//!     -d '{"name": "John Doe", "email": "john@example.com"}'"#;
//! let request = ParsedRequest::load(curl, json!({ "token": "123456" }))?;
//! # Ok(())
//! # }
//! ```
//!
//! Using with reqwest (requires `reqwest` feature):
//!
//! ```
//! # #[cfg(feature = "reqwest")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use curl_parser::ParsedRequest;
//! use serde_json::json;
//!
//! let curl = r#"curl -X POST https://api.example.com/users \
//!     -H 'Authorization: Bearer {{ token }}' \
//!     -d '{"name": "John Doe"}'"#;
//!
//! let parsed = ParsedRequest::load(curl, json!({ "token": "123456" }))?;
//! let request = reqwest::RequestBuilder::try_from(&parsed)?;
//! let response = request.send().await?;
//! # Ok(())
//! # }
//! ```

pub(crate) mod error;
mod parser;

#[cfg(feature = "uri")]
use http::Uri;
use http::{HeaderMap, Method};

pub use error::Error;

#[derive(Debug, Clone)]
pub struct ParsedRequest {
    pub method: Method,
    #[cfg(feature = "uri")]
    pub url: Uri,
    #[cfg(not(feature = "uri"))]
    pub url: String,
    pub headers: HeaderMap,
    pub body: Vec<String>,
    pub insecure: bool,
}

impl Default for ParsedRequest {
    fn default() -> Self {
        Self {
            method: Method::GET,
            #[cfg(feature = "uri")]
            url: Uri::default(),
            #[cfg(not(feature = "uri"))]
            url: String::new(),
            headers: HeaderMap::with_capacity(8), // Pre-allocate for typical header count
            body: Vec::with_capacity(4),          // Pre-allocate for typical body parts count
            insecure: false,
        }
    }
}
