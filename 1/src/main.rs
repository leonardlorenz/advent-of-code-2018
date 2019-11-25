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

fn calc(buffer : String) -> i64 {
    let mut frequency : i64 = 0;

    for splt in buffer.split(',') {
        let mut next_operation : char = '~';
        let mut next_number : String = String::new();

        for c in splt.chars() {
            if c == ' ' || c.is_alphabetic() {
                continue;
            }

            if c == '-' {
                next_operation = '-';
            } else if c == '+' {
                next_operation = '+';
            }

            if c.is_digit(10) {
                next_number.push(c);
            }
        }

        println!("{}{}", next_operation, next_number);

        if next_number != "" && next_operation != '~' {
            if next_operation == '+' {
                frequency += next_number.parse::<i64>().unwrap();
            } else if next_operation == '-' {
                frequency -= next_number.parse::<i64>().unwrap();
            }
        }
    }
    return frequency;
}
