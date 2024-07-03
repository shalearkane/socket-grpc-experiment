use std::io::stdin;

use voting::{voting_client::VotingClient, VotingRequest};
#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

pub mod voting {
    tonic::include_proto!("voting");
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = "/tmp/grpc.sock";

            // Connect to a Uds socket
            UnixStream::connect(path)
        }))
        .await?;

    let mut client = VotingClient::new(channel);
    loop {
        println!("\nPlease vote for a particular url");
        let mut u = String::new();
        let mut vote: String = String::new();
        println!("Please provide a url: ");
        stdin().read_line(&mut u).unwrap();
        let u = u.trim();
        println!("Please vote (d)own or (u)p: ");
        stdin().read_line(&mut vote).unwrap();
        let v = match vote.trim().to_lowercase().chars().next().unwrap() {
            'u' => 0,
            'd' => 1,
            _ => break,
        };
        let request = tonic::Request::new(VotingRequest {
            url: String::from(u),
            vote: v,
        });
        let response = client.vote(request).await?;
        println!("Got: '{}' from service", response.into_inner().confirmation);
    }
    Ok(())
}


#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}