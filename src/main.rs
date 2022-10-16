#![allow(unused)]

use std::io::{self, Write};

use crate::fraction::fr::Fraction;
pub mod fraction;


fn main() {
    let mut fr1 : Fraction = Fraction { num: 12, den: 25 };
    let mut fr2 : Fraction = Fraction { num: 9, den: 20 };

    let mut fr3 = fr1 - fr2;

    /*
    print!("Type a number: ");
    io::stdout().flush().unwrap();
    
    let mut val: String = String::new();
    io::stdin().read_line(&mut val).expect("Didn't receive input");

    let mut fr1 : Fraction = FractionImpl::new_val_str(&mut val.trim_end().to_string());
    //fr1.simplify();
    */

    println!("{}/{} = {}", fr1.num, fr1.den, fr1.value());
    println!("{}/{} = {}", fr2.num, fr2.den, fr2.value());
    println!("{}/{} = {}", fr3.num, fr3.den, fr3.value());
}