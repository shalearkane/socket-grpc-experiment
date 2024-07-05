use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    // Path to the Unix domain socket
    let socket_path = "/tmp/test.sock";

    // Connect to the socket
    let mut stream = UnixStream::connect(socket_path)?;

    // Data to send
    let data = b"This is some data to send";

    // Write data to the socket
    stream.write_all(data)?;

    // Buffer to store the response
    let mut buffer = [0; 1024];

    // Read response from the socket
    let bytes_read = stream.read(&mut buffer)?;

    // Print the received response
    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}
