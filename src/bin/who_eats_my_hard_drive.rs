extern crate rs_release;

#[derive(Debug)]
enum Error {
    UnknownOs,
    ReadError,
}

fn get_os_id() -> Result<String, Error> {
    match rs_release::get_os_release() {
        Err(_) => Err(Error::ReadError),
        Ok(mut os_release) => os_release.remove("ID").ok_or(Error::UnknownOs)
    }
}

fn show_fedora_packages() {
}

fn show_debian_packages() {
}

fn main() {
    match get_os_id() {
        Ok(id) => match id.as_str() {
            "fedora" => show_fedora_packages(),
            "debian" => show_debian_packages(),
            _ => println!("ERROR: {:?}", Error::UnknownOs),
        },
        Err(e) => println!("ERROR: {:?}", e),
    }
}
