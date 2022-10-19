pub mod fraction;
use crate::operation::fraction::fr::Fraction;

pub mod op {
    use super::fraction::fr::Fraction;

    // make an operation and return the result as a fraction
    pub fn make_operation(str : String) -> Fraction {
        let mut fr_vec: Vec<Fraction> = Vec::new(); // list of fractions
        let mut op_vec: Vec<char> = Vec::new();     // list of operators
    
        let mut first_sub = 0;   // every fraction will be created from this index to the next found operator character
        let mut br_found = false; // true when a bracket is found
        let mut i = 0; // next index (from string)
        while i < str.len() {
            let ch = str.as_bytes()[i] as char;
            match ch {
                '(' => {
                    let mut br_count = 1;
                    let mut br_ind = i;
                    while br_count > 0 { // loop until the proper end bracket is found
                        br_ind += 1;
                        let ch2 = str.as_bytes()[br_ind] as char;
                        match ch2 {
                            '(' => { br_count += 1 }
                            ')' => { br_count -= 1 }
                            '\r' => { break; } // end of the string
                            _ => {}
                        }
                    }
                    let str_part = str[(i + 1)..br_ind].to_string() + "\r\n"; // get the string part in brackets
                    i = br_ind;
                    first_sub = i + 1;

                    let new_fraction = make_operation(str_part); // get the result of the operation from brackets
                    fr_vec.push(new_fraction);
                    br_found = true;
                }
                '*' | '/' | '+' | '-' | '\r' => {
                    op_vec.push(ch);
                    if !br_found { // add a new fraction to the vector; if a bracket was found, it means it was already added to the vector
                        let mut str_part = str[first_sub..i].to_string();
                        fr_vec.push(Fraction::new_val_str(&mut str_part));
                    } else {
                        br_found = false;
                    }
                    first_sub = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
    
        // do the multiplication and division in the first place
        let mut ind = 0;
        while ind < op_vec.len() {
            match op_vec[ind] {
                '*' | '/' => {
                    let new_fr = if op_vec[ind] == '*' {
                            fr_vec[ind] * fr_vec[ind + 1]
                        } else {
                            fr_vec[ind] / fr_vec[ind + 1]
                        };
                    fr_vec[ind] = new_fr;
                    op_vec.remove(ind);
                    fr_vec.remove(ind + 1);
                }
                _ => { ind += 1; }
            }
        }
    
        // then do addition and substraction
        let mut ind = 0;
        while ind < op_vec.len() {
            match op_vec[ind] {
                '+' | '-' => {
                    let new_fr = if op_vec[ind] == '+' {
                            fr_vec[ind] + fr_vec[ind + 1]
                        } else {
                            fr_vec[ind] - fr_vec[ind + 1]
                        };
                    fr_vec[ind] = new_fr;
                    op_vec.remove(ind);
                    fr_vec.remove(ind + 1);
                }
                _ => { ind += 1; }
            }
        }

        fr_vec[0] // after all the operations the fractions vector should contain only one fraction
    }
    
}

