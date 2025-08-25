# curl-parser

[![Crates.io](https://img.shields.io/crates/v/curl-parser.svg)](https://crates.io/crates/curl-parser)
[![Documentation](https://docs.rs/curl-parser/badge.svg)](https://docs.rs/curl-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for parsing curl commands into structured HTTP request objects.

[中文文档](./README_zh.md)

## Overview

Many APIs provide curl examples to help users get started quickly. This crate bridges the gap between curl command examples and Rust code by parsing curl commands into structured `ParsedRequest` objects that can be easily converted to HTTP requests.

## Features

- **Parse curl commands** into structured Rust objects
- **Template support** for dynamic values (e.g., API tokens)
- **Automatic conversions** for common patterns
- **reqwest integration** (optional)
- **High performance** with optimized parsing

### Supported curl Options

- `-X, --request` - HTTP method
- `-H, --header` - HTTP headers
- `-d, --data` - Request body
- `-u` - Basic authentication
- `-L, --location` - Follow redirects
- `-k, --insecure` - Skip SSL verification

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
curl-parser = "0.6"
```

### Feature Flags

- `reqwest` (enabled by default) - Enables conversion to `reqwest::RequestBuilder`
- `uri` (enabled by default) - Parses URLs into `http::Uri` type

To use without default features:

```toml
[dependencies]
curl-parser = { version = "0.6", default-features = false }
```

## Quick Start

### Basic Usage

```rust
use curl_parser::ParsedRequest;
use std::str::FromStr;

let curl_cmd = "curl https://api.example.com/users";
let request = ParsedRequest::from_str(curl_cmd)?;

println!("Method: {}", request.method);
println!("URL: {}", request.url);
```

### With Template Variables

```rust
use curl_parser::ParsedRequest;
use serde_json::json;

let curl_cmd = r#"curl -X POST https://api.github.com/repos \
    -H "Authorization: Bearer {{ token }}" \
    -d '{"name": "{{ repo_name }}"}"#;

let context = json!({
    "token": "your_github_token",
    "repo_name": "my-new-repo"
});

let request = ParsedRequest::load(curl_cmd, context)?;
```

### Convert to reqwest

```rust
use curl_parser::ParsedRequest;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curl_cmd = "curl https://api.github.com/users/rust-lang";
    let parsed = ParsedRequest::from_str(curl_cmd)?;

    // Convert to reqwest::RequestBuilder
    let request: reqwest::RequestBuilder = parsed.try_into()?;

    // Send the request
    let response = request.send().await?;
    println!("Status: {}", response.status());

    Ok(())
}
```

## Examples

### POST with JSON Body

```rust
let curl = r#"curl -X POST https://api.example.com/users \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer token123' \
    -d '{"name": "John Doe", "email": "john@example.com"}"#;

let request = ParsedRequest::from_str(curl)?;
assert_eq!(request.method, Method::POST);
assert_eq!(request.body, vec![r#"{"name": "John Doe", "email": "john@example.com"}"#]);
```

### Basic Authentication

```rust
let curl = r#"curl https://api.stripe.com/v1/charges \
    -u sk_test_1234: \
    -H "Stripe-Version: 2022-11-15""#;

let request = ParsedRequest::from_str(curl)?;
// The -u flag is automatically converted to Authorization: Basic header
```

### Form Data

```rust
let curl = r#"curl -X POST https://httpbin.org/post \
    -d 'name=John' \
    -d 'age=30' \
    -d 'city=New York'"#;

let request = ParsedRequest::from_str(curl)?;
// Multiple -d flags are collected and form-urlencoded
```

## Advanced Features

### Escaped JSON in Headers

The parser correctly handles escaped JSON in headers:

```rust
let curl = r#"curl https://api.example.com \
    -H "X-Custom-Data: {\"key\":\"value\",\"nested\":{\"data\":true}}"#;

let request = ParsedRequest::from_str(curl)?;
// The escaped JSON is properly unescaped in the header value
```

### Automatic Method Inference

If a request body is provided without an explicit method, POST is automatically used:

```rust
let curl = r#"curl https://api.example.com -d '{"data": "value"}'"#;
let request = ParsedRequest::from_str(curl)?;
assert_eq!(request.method, Method::POST); // Automatically set to POST
```

### Default Headers

The parser automatically adds common default headers:

- `Accept: */*` if not specified
- `Content-Type: application/x-www-form-urlencoded` for form data

## Performance

This crate is optimized for performance with:

- Cached template environment (60%+ improvement for template operations)
- Pre-allocated collections for common sizes
- Efficient string operations using byte-level matching
- Optimized grammar rules for the Pest parser

Run benchmarks with:

```bash
cargo bench --bench parsing_benchmark
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
cargo fmt
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Pest](https://pest.rs/) parser
- Template support via [minijinja](https://github.com/mitsuhiko/minijinja)
- Inspired by the need to quickly convert API documentation to working code
