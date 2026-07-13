use std::net::UdpSocket;
use std::{io, str};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Failed to bind");

    socket
        .connect("127.0.0.1:8888")
        .expect("Failed to connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        socket
            .send(input.as_bytes())
            .expect("Failed to write to server");

        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");

        print!(
            "{}",
            str::from_utf8(&buffer).expect("Couldn't write buffer as string")
        );
    }
}
