use std::string::String;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let frequency = calc(buffer);

    println!("{}", frequency);
}

fn calc(buffer : String) -> u64 {
    let twice_amount : u64 = 0;
    let thrice_amount : u64 = 0;
    let checksum : u64 = 0;

    for splt in buffer.split(',') {
        for c in splt.chars() {
        }

    }
    return checksum;
}
