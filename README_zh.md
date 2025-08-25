# curl-parser

[![Crates.io](https://img.shields.io/crates/v/curl-parser.svg)](https://crates.io/crates/curl-parser)
[![Documentation](https://docs.rs/curl-parser/badge.svg)](https://docs.rs/curl-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个将 curl 命令解析为结构化 HTTP 请求对象的 Rust 库。

[English Documentation](./README.md)

## 概述

现在大多数 API 都提供 curl 示例，让用户可以快速试用 API，无需任何入门障碍。但是将这些示例转换为 Rust 代码需要时间来理解和转换。这个 crate 可以将 curl 命令转换为 Rust 代码，填补了 curl 命令示例和 Rust 代码之间的空白。

## 特性

- **解析 curl 命令** 为结构化的 Rust 对象
- **模板支持** 用于动态值（例如 API 令牌）
- **自动转换** 常见模式
- **reqwest 集成**（可选）
- **高性能** 优化的解析器

### 支持的 curl 选项

- `-X, --request` - HTTP 方法
- `-H, --header` - HTTP 头部
- `-d, --data` - 请求体
- `-u` - 基本认证
- `-L, --location` - 跟随重定向
- `-k, --insecure` - 跳过 SSL 验证

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
curl-parser = "0.6"
```

### 功能标志

- `reqwest`（默认启用）- 启用转换为 `reqwest::RequestBuilder`
- `uri`（默认启用）- 将 URL 解析为 `http::Uri` 类型

不使用默认功能：

```toml
[dependencies]
curl-parser = { version = "0.6", default-features = false }
```

## 快速开始

### 基本用法

```rust
use curl_parser::ParsedRequest;
use std::str::FromStr;

let curl_cmd = "curl https://api.example.com/users";
let request = ParsedRequest::from_str(curl_cmd)?;

println!("方法: {}", request.method);
println!("URL: {}", request.url);
```

### 使用模板变量

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

### 转换为 reqwest

```rust
use curl_parser::ParsedRequest;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curl_cmd = "curl https://api.github.com/users/rust-lang";
    let parsed = ParsedRequest::from_str(curl_cmd)?;

    // 转换为 reqwest::RequestBuilder
    let request: reqwest::RequestBuilder = parsed.try_into()?;

    // 发送请求
    let response = request.send().await?;
    println!("状态: {}", response.status());

    Ok(())
}
```

## 示例

### POST 请求带 JSON 体

```rust
let curl = r#"curl -X POST https://api.example.com/users \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer token123' \
    -d '{"name": "John Doe", "email": "john@example.com"}"#;

let request = ParsedRequest::from_str(curl)?;
assert_eq!(request.method, Method::POST);
assert_eq!(request.body, vec![r#"{"name": "John Doe", "email": "john@example.com"}"#]);
```

### 基本认证

```rust
let curl = r#"curl https://api.stripe.com/v1/charges \
    -u sk_test_1234: \
    -H "Stripe-Version: 2022-11-15""#;

let request = ParsedRequest::from_str(curl)?;
// -u 标志会自动转换为 Authorization: Basic 头部
```

### 表单数据

```rust
let curl = r#"curl -X POST https://httpbin.org/post \
    -d 'name=John' \
    -d 'age=30' \
    -d 'city=New York'"#;

let request = ParsedRequest::from_str(curl)?;
// 多个 -d 标志会被收集并进行 form-urlencoded 编码
```

## 高级功能

### 头部中的转义 JSON

解析器正确处理头部中的转义 JSON：

```rust
let curl = r#"curl https://api.example.com \
    -H "X-Custom-Data: {\"key\":\"value\",\"nested\":{\"data\":true}}"#;

let request = ParsedRequest::from_str(curl)?;
// 转义的 JSON 会在头部值中被正确地反转义
```

### 自动方法推断

如果提供了请求体但没有明确指定方法，会自动使用 POST：

```rust
let curl = r#"curl https://api.example.com -d '{"data": "value"}'"#;
let request = ParsedRequest::from_str(curl)?;
assert_eq!(request.method, Method::POST); // 自动设置为 POST
```

### 默认头部

解析器会自动添加常见的默认头部：

- 如果未指定，添加 `Accept: */*`
- 为表单数据添加 `Content-Type: application/x-www-form-urlencoded`

## 性能

此 crate 针对性能进行了优化：

- 缓存的模板环境（模板操作性能提升 60%+）
- 为常见大小预分配集合
- 使用字节级匹配的高效字符串操作
- 优化的 Pest 解析器语法规则

运行基准测试：

```bash
cargo bench --bench parsing_benchmark
```

### 性能提升数据

最近的优化带来了显著的性能提升：

- **复杂解析操作**：提升 59-66%
- **模板渲染**：通过缓存获得显著加速
- **表单体处理**：提升 7.4%

## 开发

### 构建

```bash
cargo build
```

### 测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt
```

## 使用场景

这个库特别适合以下场景：

1. **API 客户端开发**：快速将 API 文档中的 curl 示例转换为可用的 Rust 代码
2. **测试和调试**：将 curl 命令转换为程序化的 HTTP 请求进行自动化测试
3. **代码生成**：从 curl 示例生成 Rust 代码
4. **学习工具**：帮助理解 curl 命令如何映射到 HTTP 请求

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 贡献指南

1. Fork 这个仓库
2. 创建你的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 许可证

本项目使用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- 使用 [Pest](https://pest.rs/) 解析器构建
- 通过 [minijinja](https://github.com/mitsuhiko/minijinja) 提供模板支持
- 灵感来自于快速将 API 文档转换为可工作代码的需求

## 相关项目

如果你喜欢这个项目，你可能也会对以下项目感兴趣：

- [reqwest](https://github.com/seanmonstar/reqwest) - Rust 的 HTTP 客户端
- [http](https://github.com/hyperium/http) - Rust 的 HTTP 类型
- [pest](https://pest.rs/) - Rust 的通用解析器
