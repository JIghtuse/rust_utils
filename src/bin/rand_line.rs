extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use rand::Rng;

// Prints random line from a file
fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: {} filename.txt", args[0]);
    }

    let file = File::open(&args[1]).expect("Cannot open input file");
    let lines = BufReader::new(file).lines();
    let lines : Vec<String> = lines.map(|line| line.expect("no line")).collect();

    if let Some(line) = rand::thread_rng().choose(&lines) {
        println!("{}", line);
    } else {
        panic!("cannot get random line from input file");
    }
}
