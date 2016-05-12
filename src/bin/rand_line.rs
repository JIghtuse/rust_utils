#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::process;
use std::path::Path;
use rand::Rng;

fn get_random_line<P: AsRef<Path>>(filename: P) -> Option<String> {
    if let Ok(file) = File::open(filename) {
        let lines = BufReader::new(file).lines();
        let lines : Vec<String> = lines.map(|line| line.unwrap()).collect();
        rand::thread_rng().choose(&lines).cloned()
    } else {
        None
    }
}

// Prints random line from a file
fn main() {
    let mut args = env::args();
    let program_name = args.next();

    if let Some(input_file) = args.next() {
        if let Some(line) = get_random_line(&input_file) {
            println!("{}", line);
        } else {
            println!("[ERROR] cannot get random line from file {}", input_file);
        }
    } else {
        println!("usage: {} filename.txt", program_name.unwrap());
        process::exit(1);
    }
}
