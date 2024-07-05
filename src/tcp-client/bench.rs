use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

fn tcp_benchmark(c: &mut Criterion) {
    c.bench_function("tcp", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .enable_io()
                .build()
                .unwrap(),
        )
        .iter_custom(|iters| async move {
            let client = Client::new();
            let start: Instant = Instant::now();

            for _i in 0..iters {
                let url = "http:localhost:8080/rest";
                let payload = json!({
                    "url": "something.com",
                    "vote": 1
                });
                client.post(url).json(&payload).send().await.unwrap();
            }

            start.elapsed()
        })
    });
}

criterion_group!(resp_bench, tcp_benchmark);
criterion_main!(resp_bench);
