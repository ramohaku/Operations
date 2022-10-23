#![allow(unused)]

use std::io::{self, Write};

pub mod operation;
use crate::operation::{fraction::fr::Fraction, op::make_operation};

fn main() {
    loop {
        print!("Type an operation: ");
        io::stdout().flush().unwrap();
        let mut val: String = String::new();
        io::stdin().read_line(&mut val).expect("Didn't receive input");
    
        if val == "\r\n" { // break the loop if user didn't type anything
            break;
        }

        let result = make_operation(val);
        println!(" = {}/{} = {}", result.num, result.den, result.value());
    }
}