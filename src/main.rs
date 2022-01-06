use std::io;
use std::io::prelude::*;
use urlencoding::encode;

fn main() {
    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        let encoded = encode(&l);
        println!("{}", encoded);
    }
}
