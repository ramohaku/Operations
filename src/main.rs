#![allow(unused)]

use std::io::{self, Write};

pub mod operation;
use crate::operation::{fraction::fr::Fraction, op::make_operation};

fn main() {
    print!("Type an operation: ");
    io::stdout().flush().unwrap();
    let mut val: String = String::new();
    io::stdin().read_line(&mut val).expect("Didn't receive input");

    let result = make_operation(val);
    println!(" = {}/{} = {}", result.num, result.den, result.value());
}