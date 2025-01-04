# CURL Parser

Nowadays, most of the APIs provide CURL examples to allow users to try out the APIs without any entry barriers, but it takes time to digest the examples and convert them into Rust code. This crate could convert CURL commands into Rust code.

At the moment, it supports `-X`, `-H`, `-d` and `-u` options since these are the most widely used ones.

## Usage

```rust
let input = r#"curl \
    -X PATCH \
    -d '{"visibility":"private"}' \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer {{ token }}"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/email/visibility "#;
let context = json!({ "token": "abcd1234" });
let parsed = curl_parser::ParsedRequest::load(input, Some(context)) ?;
println!("{:#?}", parsed);
let req: reqwest::RequestBuilder = parsed.try_into()?;
let res = req.send().await?;
assert_eq!(res.status(), 200);
```

By default, `reqwest` is enabled so that you can convert `ParsedRequest` into a `reqwest::RequestBuilder`. If you don't want to use `reqwest`, you can disable the default features.
