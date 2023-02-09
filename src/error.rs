use crate::parser::Rule;
use http::Method;
use snafu::Snafu;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to parse input string with pest rules"))]
    ParseRule {
        #[snafu(source(from(pest::error::Error<Rule>, Box::new)))]
        source: Box<pest::error::Error<Rule>>,
    },
    #[snafu(display("Expect a {label}. But value is {value}"))]
    ExpectValue { label: &'static str, value: String },
    #[snafu(display("Failed to parse HTTP method"))]
    ParseMethod { source: http::method::InvalidMethod },
    #[snafu(display("Unsupported HTTP method: {method}"))]
    UnsupportedMethod { method: Method },
    #[snafu(display("URL is required for http call"))]
    RequiredUrl,
    #[snafu(display("Value is required"))]
    RequiredValue,
    #[snafu(display("Unsupported type: {value}"))]
    UnsupportedType { value: String },
    #[snafu(display("Unsupported attr {name}: {value}"))]
    UnsupportedAttr { name: String, value: String },

    #[snafu(display("Failed to parse URL"))]
    ParseUrl { source: http::uri::InvalidUri },
    #[snafu(display("Failed to parse header name"))]
    ParseHeaderName {
        source: http::header::InvalidHeaderName,
    },
    #[snafu(display("Failed to parse header value"))]
    ParseHeaderValue {
        source: http::header::InvalidHeaderValue,
    },

    #[snafu(display("Failed to render request template"))]
    Render { source: minijinja::Error },
}
