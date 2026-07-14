use std::{env, net::ToSocketAddrs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <hostname>");
        return;
    }

    let hostname = &args[1];

    let addresses = (hostname.as_str(), 0)
        .to_socket_addrs()
        .expect("Could not resolve hostname");

    for address in addresses {
        println!("{}", address.ip());
    }
}
