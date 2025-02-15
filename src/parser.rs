use crate::{error::*, ParsedRequest};
use base64::{engine::general_purpose::STANDARD, Engine};
use http::{
    header::{HeaderName, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use minijinja::Environment;
use pest::Parser as _;
use pest_derive::Parser;
use serde::Serialize;
use snafu::ResultExt;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[grammar = "src/curl.pest"]
pub struct CurlParser;

fn parse_input(input: &str) -> Result<ParsedRequest> {
    let pairs = CurlParser::parse(Rule::input, input).context(ParseRuleSnafu)?;
    let mut parsed = ParsedRequest::default();
    for pair in pairs {
        match pair.as_rule() {
            Rule::method => {
                let method_str = remove_quote(pair.as_str());
                let method = method_str.parse().context(ParseMethodSnafu)?;
                parsed.method = method;
            }
            Rule::url => {
                let url = pair.into_inner().as_str();

                // if empty scheme set curl defaults to HTTP
                #[cfg(feature = "uri")]
                let url = if url.contains("://") {
                    url.parse().context(ParseUrlSnafu)?
                } else {
                    format!("http://{url}").parse().context(ParseUrlSnafu)?
                };
                #[cfg(not(feature = "uri"))]
                let url = if url.contains("://") {
                    url.to_string()
                } else {
                    format!("http://{url}/")
                };

                parsed.url = url;
            }
            Rule::location => {
                let s = pair
                    .into_inner()
                    .next()
                    .expect("location string must be present")
                    .as_str();
                #[cfg(feature = "uri")]
                let location = s.parse().context(ParseUrlSnafu)?;
                #[cfg(not(feature = "uri"))]
                let location = s.to_string();
                parsed.url = location;
            }
            Rule::header => {
                let s = pair
                    .into_inner()
                    .next()
                    .expect("header string must be present")
                    .as_str();
                let mut kv = s.splitn(2, ':');
                let name = kv.next().expect("key must present").trim();
                let value = kv.next().expect("value must present").trim();
                parsed.headers.insert(
                    HeaderName::from_str(name).context(ParseHeaderNameSnafu)?,
                    HeaderValue::from_str(value).context(ParseHeaderValueSnafu)?,
                );
            }
            Rule::auth => {
                let s = pair
                    .into_inner()
                    .next()
                    .expect("header string must be present")
                    .as_str();
                let basic_auth = format!("Basic {}", STANDARD.encode(s.as_bytes()));
                parsed.headers.insert(
                    AUTHORIZATION,
                    basic_auth.parse().context(ParseHeaderValueSnafu)?,
                );
            }
            Rule::body => {
                let s = pair.as_str().trim();
                let s = remove_quote(s);
                parsed.body.push(s.into());
            }
            Rule::ssl_verify_option => {
                parsed.insecure = true;
            }
            Rule::EOI => break,
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    if parsed.headers.get(CONTENT_TYPE).is_none() && !parsed.body.is_empty() {
        parsed.headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
    }
    if parsed.headers.get(ACCEPT).is_none() {
        parsed
            .headers
            .insert(ACCEPT, HeaderValue::from_static("*/*"));
    }
    if !parsed.body.is_empty() && parsed.method == Method::GET {
        parsed.method = Method::POST
    }
    Ok(parsed)
}

impl ParsedRequest {
    pub fn load(input: &str, context: impl Serialize) -> Result<Self> {
        let env = Environment::new();
        let input = env.render_str(input, context).context(RenderSnafu)?;
        parse_input(&input)
    }

    pub fn body(&self) -> Option<String> {
        if self.body.is_empty() {
            return None;
        }

        match self.headers.get(CONTENT_TYPE) {
            Some(content_type) if content_type == "application/x-www-form-urlencoded" => {
                Some(self.form_urlencoded())
            }
            Some(content_type) if content_type == "application/json" => self.body.last().cloned(),
            v => unimplemented!("Unsupported content type: {:?}", v),
        }
    }

    fn form_urlencoded(&self) -> String {
        let mut encoded = form_urlencoded::Serializer::new(String::new());
        for item in &self.body {
            let mut kv = item.splitn(2, '=');
            let key = kv.next().expect("key must present");
            let value = kv.next().expect("value must present");
            encoded.append_pair(remove_quote(key), remove_quote(value));
        }
        encoded.finish()
    }
}

impl FromStr for ParsedRequest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_input(s)
    }
}

#[cfg(feature = "reqwest")]
impl TryFrom<&ParsedRequest> for reqwest::RequestBuilder {
    type Error = reqwest::Error;

    fn try_from(req: &ParsedRequest) -> Result<Self, Self::Error> {
        let body = req.body();
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(req.insecure)
            .build()?;

        let req_builder = client
            .request(req.method.clone(), req.url.to_string())
            .headers(req.headers.clone());

        let req = if let Some(body) = body {
            req_builder.body(body)
        } else {
            req_builder
        };

        Ok(req)
    }
}

fn remove_quote(s: &str) -> &str {
    match (&s[0..1], &s[s.len() - 1..]) {
        ("'", "'") => &s[1..s.len() - 1],
        ("\"", "\"") => &s[1..s.len() - 1],
        _ => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use http::{header::ACCEPT, Method};
    use serde_json::json;

    #[test]
    fn parse_curl_1_should_work() -> Result<()> {
        let input = r#"curl \
          -X PATCH \
          -d '{"visibility":"private"}' \
          -H "Accept: application/vnd.github+json" \
          -H "Authorization: Bearer {{ token }}"\
          -H "X-GitHub-Api-Version: 2022-11-28" \
          https://api.github.com/user/email/visibility "#;
        let parsed = ParsedRequest::load(input, json!({ "token": "abcd1234" }))?;
        assert_eq!(parsed.method, Method::PATCH);
        assert_eq!(
            parsed.url.to_string(),
            "https://api.github.com/user/email/visibility"
        );
        assert_eq!(
            parsed.headers.get(ACCEPT),
            Some(&HeaderValue::from_static("application/vnd.github+json"))
        );
        assert_eq!(parsed.body, vec!["{\"visibility\":\"private\"}"]);

        Ok(())
    }

    #[test]
    fn parse_curl_2_should_work() -> Result<()> {
        let input = r#"curl \
        -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer {{ token }}"\
        -H "X-GitHub-Api-Version: 2022-11-28" \
        -L "https://api.github.com/user/emails" \
        -d '{"emails":["octocat@github.com","mona@github.com","octocat@octocat.org"]}'"#;
        let parsed = ParsedRequest::load(input, json!({ "token": "abcd1234" }))?;
        assert_eq!(parsed.method, Method::POST);
        assert_eq!(parsed.url.to_string(), "https://api.github.com/user/emails");
        assert_eq!(
            parsed.headers.get(AUTHORIZATION),
            Some(&HeaderValue::from_static("Bearer abcd1234"))
        );
        assert_eq!(
            parsed.body,
            vec![r#"{"emails":["octocat@github.com","mona@github.com","octocat@octocat.org"]}"#]
        );
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_3_should_work() -> Result<()> {
        let input = r#"curl https://api.stripe.com/v1/charges \
        -u {{ key }}: \
        -H "Stripe-Version: 2022-11-15""#;

        let parsed =
            ParsedRequest::load(input, json!({ "key": "sk_test_4eC39HqLyjWDarjtT1zdp7dc" }))?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "https://api.stripe.com/v1/charges");
        assert_eq!(
            parsed.headers.get(AUTHORIZATION),
            Some(&HeaderValue::from_static(
                "Basic c2tfdGVzdF80ZUMzOUhxTHlqV0Rhcmp0VDF6ZHA3ZGM6"
            ))
        );

        #[cfg(feature = "reqwest")]
        {
            let req = reqwest::RequestBuilder::try_from(&parsed)?;
            let res = req.send().await?;
            assert_eq!(res.status(), 200);
            let _body = res.text().await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_4_should_work() -> Result<()> {
        let input = r#"curl "https://ifconfig.me/""#;

        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "https://ifconfig.me/");

        #[cfg(feature = "reqwest")]
        {
            let req = reqwest::RequestBuilder::try_from(&parsed)?;
            let res = req.send().await?;
            assert_eq!(res.status(), 200);
            let _body = res.text().await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_5_should_work() -> Result<()> {
        let input = r#"curl 'ifconfig.me'"#;

        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "http://ifconfig.me/");

        #[cfg(feature = "reqwest")]
        {
            let req = reqwest::RequestBuilder::try_from(&parsed)?;
            let res = req.send().await?;
            assert_eq!(res.status(), 200);
            let _body = res.text().await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_insecure_should_work() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"#this is good
        curl -k 'https://example.com/'"#;

        let parsed: ParsedRequest = input.parse()?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "https://example.com/");
        assert!(parsed.insecure);
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_body_should_work() -> Result<()> {
        let input = r#"curl --location https://example.com --header 'Content-Type: application/json' -d '{"-name":"--John"," --age":30}'"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::POST);
        assert_eq!(parsed.body, vec!["{\"-name\":\"--John\",\" --age\":30}"]);
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_data_raw_should_work() -> Result<()> {
        let input = r#"curl --location 'https://httpbin.org/post' --header 'Content-Type: application/json' --data-raw '{"name":"John Doe","age":30}'"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::POST);
        assert_eq!(parsed.body, vec![r#"{"name":"John Doe","age":30}"#]);
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_unquoted_headers_should_work() -> Result<()> {
        let input =
            r#"curl -X GET https://httpbin.org/get -H accept: application/json -H X-API-KEY: TEST"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "https://httpbin.org/get");
        assert_eq!(
            parsed.headers.get("accept"),
            Some(&HeaderValue::from_static("application/json"))
        );
        assert_eq!(
            parsed.headers.get("X-API-KEY"),
            Some(&HeaderValue::from_static("TEST"))
        );
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_multiline_json_should_work() -> Result<()> {
        let input = r#"curl -X 'POST' \
  'https://httpbin.org/post' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "metadata": {
    "group_by": "ad_name",
    "limit": 10,
    "page": 1,
    "sort": {
      "by": "ad_name",
      "order": "asc"
    }
  },
  "query": [
  ]
}'"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::POST);
        assert_eq!(parsed.url.to_string(), "https://httpbin.org/post");
        assert_eq!(
            parsed.headers.get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("application/json"))
        );
        assert_eq!(
            parsed.headers.get(ACCEPT),
            Some(&HeaderValue::from_static("application/json"))
        );
        let expected_body = r#"{
  "metadata": {
    "group_by": "ad_name",
    "limit": 10,
    "page": 1,
    "sort": {
      "by": "ad_name",
      "order": "asc"
    }
  },
  "query": [
  ]
}"#;
        assert_eq!(parsed.body, vec![expected_body]);

        #[cfg(feature = "reqwest")]
        {
            let req = reqwest::RequestBuilder::try_from(&parsed)?;
            let res = req.send().await?;
            assert_eq!(res.status(), 200);
            let body = res.text().await?;
            assert!(body.contains("ad_name")); // Verify the JSON was sent correctly
        }

        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_no_protocol_should_work() -> Result<()> {
        let input = r#"curl -X GET httpbin.org/get"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(parsed.url.to_string(), "http://httpbin.org/get");
        Ok(())
    }

    #[tokio::test]
    async fn parse_curl_with_special_chars_should_work() -> Result<()> {
        let input = r#"curl 'https://httpbin.org/anything/test.php?key1=value1&key2=value2~!@%24%25%5E*&user=john.doe+test@example.com'"#;
        let parsed = ParsedRequest::from_str(input)?;
        assert_eq!(parsed.method, Method::GET);
        assert_eq!(
        parsed.url.to_string(),
        "https://httpbin.org/anything/test.php?key1=value1&key2=value2~!@%24%25%5E*&user=john.doe+test@example.com"
    );
        Ok(())
    }
}
