use criterion::{criterion_group, criterion_main, Criterion};

use std::time::Instant;

#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;
use voting::{voting_client::VotingClient, VotingRequest};

pub mod voting {
    tonic::include_proto!("voting");
}

fn grpc_benchmark(c: &mut Criterion) {
    c.bench_function("grpc", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap(),
        )
        .iter_custom(|iters| {
            async move {
                let channel = Endpoint::try_from("http://[::]:50051")
                    .unwrap()
                    .connect_with_connector(service_fn(|_: Uri| {
                        let path = "/tmp/grpc.sock";

                        // Connect to a Uds socket
                        UnixStream::connect(path)
                    }))
                    .await
                    .unwrap();

                let mut client = VotingClient::new(channel);
                let start: Instant = Instant::now();

                for _i in 0..iters {
                    let request = tonic::Request::new(VotingRequest {
                        url: "something.com".to_string(),
                        vote: 1,
                    });
                    client.vote(request).await.unwrap();
                }
                start.elapsed()
            }
        })
    });
}

criterion_group!(resp_bench, grpc_benchmark);
criterion_main!(resp_bench);
