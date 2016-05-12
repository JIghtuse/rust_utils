use std::env;
use std::net::IpAddr;
use std::process::exit;

fn main() {
    let mut args = env::args();
    let program_name = args.next();

    if let Some(addr) = args.next() {
        if addr.parse::<IpAddr>().is_ok() {
            exit(0);
        } else {
            exit(1);
        }
    } else {
        println!("usage: {:?} <ip_address>", program_name.unwrap());
        exit(1);
    }
}
