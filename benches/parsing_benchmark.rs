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
          -H 'Accept: application/json' \
          -H 'X-Custom-Header: value' \
          -d '{"name": "John Doe", "email": "john@example.com"}'"#,
        r#"curl -X PATCH \
          -d '{"visibility":"private"}' \
          -H "Accept: application/vnd.github+json" \
          -H "Authorization: Bearer {{ token }}"\
          -H "X-GitHub-Api-Version: 2022-11-28" \
          -H "User-Agent: MyApp/1.0" \
          -H "X-Request-ID: 12345" \
          https://api.github.com/user/email/visibility"#,
        r#"curl -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer {{ token }}"\
        -H "X-GitHub-Api-Version: 2022-11-28" \
        -H "Content-Type: application/json" \
        -H "X-Request-Source: API" \
        -L "https://api.github.com/user/emails" \
        -d '{"emails":["user1@example.com","user2@example.com","user3@example.com","user4@example.com"]}' \
        -k"#,
        r#"curl https://api.openweathermap.org/data/2.5/weather \
        -H "X-API-Key: {{ key }}" \
        -H "Accept: application/json" \
        -H "User-Agent: MyApp/2.0" \
        -H "X-Client-Version: {\"version\":\"1.0.0\"}" \
        -d q=London \
        -d units=metric \
        -d appid={{ key }}"#,
    ]
}

fn template_curl_commands() -> Vec<&'static str> {
    vec![
        r#"curl -H 'Authorization: Bearer {{ token }}' https://api.example.com/users"#,
        r#"curl -X POST -H 'Content-Type: application/json' -d '{"user_id": {{ user_id }}, "name": "{{ name }}"}' https://api.example.com/users"#,
        r#"curl -H 'API-Key: {{ api_key }}' -H 'X-User-ID: {{ user_id }}' -H 'X-Timestamp: {{ timestamp }}' https://api.example.com/data/{{ resource_id }}"#,
        r#"curl -X PUT \
          -H 'Authorization: Bearer {{ token }}' \
          -H 'Content-Type: application/json' \
          -H 'X-Client-Version: {{ version }}' \
          -d '{"settings": {"theme": "{{ theme }}", "language": "{{ language }}", "notifications": {{ notifications }}}}' \
          https://api.example.com/users/{{ user_id }}/preferences"#,
    ]
}

fn edge_case_commands() -> Vec<String> {
    vec![
        // Very long URL
        format!("curl 'https://api.example.com/very/long/path/with/many/segments/{}'",
                "segment".repeat(50)),

        // Many headers (20 headers)
        format!("curl https://api.example.com/headers {}",
            (0..20)
                .map(|i| format!("-H 'X-Custom-Header-{}: value-{}'", i, i))
                .collect::<Vec<_>>()
                .join(" ")),

        // Large JSON body
        r#"curl -X POST -H 'Content-Type: application/json' -d '{"data": "very long data string that represents a typical large payload that might be sent in real world applications with lots of nested objects and arrays and various data types including strings numbers and booleans", "items": [{"id": 1, "name": "item1", "description": "A very detailed description"}, {"id": 2, "name": "item2", "description": "Another detailed description"}], "metadata": {"timestamp": "2023-01-01T00:00:00Z", "version": "1.0", "author": "system"}}' https://api.example.com/data"#.to_string(),

        // Complex form data
        r#"curl -X POST -H 'Content-Type: application/x-www-form-urlencoded' \
          -d 'field1=value1' \
          -d 'field2=value2' \
          -d 'field3=value3' \
          -d 'field4=value4' \
          -d 'field5=value5' \
          -d 'field6=value6' \
          -d 'field7=value7' \
          -d 'field8=value8' \
          -d 'field9=value9' \
          -d 'field10=value10' \
          https://api.example.com/form"#.to_string(),
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
                    "key": "api_key_12345"
                });
                ParsedRequest::load(black_box(cmd), black_box(&context)).unwrap()
            })
        });
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_template_rendering(c: &mut Criterion) {
    let commands = template_curl_commands();
    let context = json!({
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        "api_key": "api_key_12345",
        "user_id": 12345,
        "name": "John Doe",
        "timestamp": "2023-01-01T00:00:00Z",
        "resource_id": "resource_123",
        "version": "1.0.0",
        "theme": "dark",
        "language": "en",
        "notifications": true
    });

    let mut group = c.benchmark_group("template_rendering");

    for (i, cmd) in commands.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("render_and_parse", i), cmd, |b, cmd| {
            b.iter(|| ParsedRequest::load(black_box(cmd), black_box(&context)).unwrap())
        });
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_edge_cases(c: &mut Criterion) {
    let commands = edge_case_commands();
    let mut group = c.benchmark_group("edge_cases");

    for (i, cmd) in commands.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("parse_edge_case", i), cmd, |b, cmd| {
            b.iter(|| ParsedRequest::from_str(black_box(cmd.as_str())).unwrap())
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
