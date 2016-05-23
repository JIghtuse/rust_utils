extern crate libloading;

use std::env;
use std::process;
use libloading::{Library, Symbol};

fn execute(library_path: &str, function: &str, argument: f64) -> Result<f64, std::io::Error> {
    let lib = try!(Library::new(library_path));
    let f: Symbol<extern "C" fn(f64) -> f64> = unsafe { try!(lib.get(function.as_bytes())) };

    Ok(f(argument))
}

fn main() {
    let mut args = env::args();
    let program_name = args.next();
    let library_path = args.next();
    let function_name = args.next();
    let argument_str = args.next();

    if program_name.is_none() || library_path.is_none() || argument_str.is_none() {
        println!("usage: {} library_path function_name argument",
                 program_name.unwrap());
        process::exit(1);
    }

    let library_path = library_path.unwrap();
    let function_name = function_name.unwrap();
    let argument_str = argument_str.unwrap();

    let argument = argument_str.parse::<f64>();
    if argument.is_err() {
        println!("Cannot convert {} to f64", argument_str);
        process::exit(1);
    }
    let argument = argument.unwrap();

    let result = execute(&library_path, &function_name, argument);
    match result {
        Ok(value) => println!("{}", value), 
        Err(e) => println!("[ERROR] {}", e), 
    }
}
