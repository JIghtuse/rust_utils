extern crate libloading;

use libloading::{Library, Symbol};

fn main() {
    let lib = Library::new("libm.so.6").unwrap();
    let sin: Symbol<extern fn(f64) -> f64> = unsafe {
        lib.get(b"sin").unwrap()
    };

    let sin_one_half = sin(0.5);
    println!("sin of 0.5: {}", sin_one_half);
}
