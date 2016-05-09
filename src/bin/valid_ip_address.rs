use std::env;
use std::net::IpAddr;
use std::process::exit;

fn main() {
    let program_name = env::args().nth(0).unwrap();

    if let Some(addr) = env::args().nth(1) {
        match addr.parse::<IpAddr>() {
            Err(_) => exit(1),
            Ok(_) => exit(0),
        }
    } else {
        println!("usage: {:?} <ip_address>", program_name);
    }
}
