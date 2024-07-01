use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixListener;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Path to the Unix domain socket
    let socket_path = Path::new("/tmp/test.sock");

    // Create the socket
    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {}", socket_path.display());

    // Loop to handle connections
    loop {
        // Accept incoming connection
        let (mut stream, _) = listener.accept()?;

        // Buffer to store received data
        let mut buffer = [0; 1024];

        // Read data from the client
        let bytes_read = stream.read(&mut buffer)?;

        // Process the received data (replace with your logic)
        println!("Client sent: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

        // Send a response (optional)
        let response = b"Data received!";
        stream.write_all(response)?;

        // Gracefully close the connection
        stream.shutdown(Shutdown::Both)?;
    }
}
