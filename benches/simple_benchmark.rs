use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use curl_parser::ParsedRequest;
use serde_json::json;
use std::hint::black_box;
use std::str::FromStr;

fn simple_curl_commands() -> Vec<&'static str> {
    vec![
        "curl https://api.example.com/users",
        "curl http://localhost:8080/health",
        "curl 'https://jsonplaceholder.typicode.com/posts/1'",
        "curl http://example.com",
        "curl -X GET https://api.github.com/user",
    ]
}

fn complex_curl_commands() -> Vec<&'static str> {
    vec![
        r#"curl -X POST https://api.example.com/users \
          -H 'Content-Type: application/json' \
          -H 'Authorization: Bearer token123' \
          -d '{"name": "John Doe", "email": "john@example.com"}'"#,
        r#"curl -X PATCH \
          -d '{"visibility":"private"}' \
          -H "Accept: application/vnd.github+json" \
          -H "Authorization: Bearer {{ token }}"\
          -H "X-GitHub-Api-Version: 2022-11-28" \
          https://api.github.com/user/email/visibility"#,
        r#"curl https://api.github.com/user/repos \
        -u {{ username }}:{{ token }} \
        -H "Accept: application/vnd.github.v3+json""#,
    ]
}

fn bench_simple_parsing(c: &mut Criterion) {
    let commands = simple_curl_commands();
    let mut group = c.benchmark_group("simple_parsing");

    for (i, cmd) in commands.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("parse", i), cmd, |b, cmd| {
            b.iter(|| ParsedRequest::from_str(black_box(cmd)).unwrap())
        });
    }
    group.finish();
}

fn bench_complex_parsing(c: &mut Criterion) {
    let commands = complex_curl_commands();
    let mut group = c.benchmark_group("complex_parsing");

    for (i, cmd) in commands.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("parse", i), cmd, |b, cmd| {
            b.iter(|| {
                let context = json!({
                    "token": "ghp_testtoken123456",
                    "username": "testuser",
                    "key": "api_key_12345"
                });
                ParsedRequest::load(black_box(cmd), black_box(&context)).unwrap()
            })
        });
    }
    group.finish();
}

fn bench_body_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("body_processing");

    // JSON body processing
    let json_request = ParsedRequest::load(
        r#"curl -X POST -H 'Content-Type: application/json' -d '{"name": "John", "age": 30}' https://api.example.com"#,
        json!({})
    ).unwrap();

    group.bench_function("json_body", |b| b.iter(|| black_box(&json_request).body()));

    // Form URL encoded body processing
    let form_request = ParsedRequest::load(
        r#"curl -X POST -d 'name=John' -d 'age=30' -d 'email=john@example.com' https://api.example.com"#,
        json!({})
    ).unwrap();

    group.bench_function("form_body", |b| b.iter(|| black_box(&form_request).body()));

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_parsing,
    bench_complex_parsing,
    bench_body_processing
);
criterion_main!(benches);
