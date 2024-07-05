use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Client;
use serde_json::json;

async fn tcp_request(client: Client) {
    let url = "http:localhost:8080/rest";
    let payload = json!({
        "url": "something.com",
        "vote": 1
    });

    client.post(url).json(&payload).send().await.unwrap();
}

fn tcp_benchmark(c: &mut Criterion) {
    let client = Client::new();
    c.bench_function(
        "tcp",
        |b: &mut criterion::Bencher<criterion::measurement::WallTime>| {
            b.iter(|| tcp_request(client.clone()))
        },
    );
}

criterion_group!(tcp_bench, tcp_benchmark);
criterion_main!(tcp_bench);
