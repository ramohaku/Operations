pub mod fraction;
use crate::operation::fraction::fr::Fraction;

pub mod op {
    use super::fraction::fr::Fraction;

    // make an operation and return the result as a fraction
    pub fn make_operation(mut st : String) -> Fraction {
        if st.as_bytes()[0] as char == '-' {
            st = "0".to_string() + &st; // add a 0 at the beginning so that negative numbers work
        }
        let mut fr_vec: Vec<Fraction> = Vec::new(); // list of fractions
        let mut op_vec: Vec<char> = Vec::new();     // list of operators
    
        let mut first_sub = 0;   // every fraction will be created from this index to the next found operator character
        let mut br_found = false; // true when a bracket is found
        let mut i = 0; // next index (from string)
        while i < st.len() {
            let ch = st.as_bytes()[i] as char;
            match ch {
                '(' => {
                    let mut br_count = 1;
                    let mut br_ind = i;
                    while br_count > 0 { // loop until the proper end bracket is found
                        br_ind += 1;
                        let ch2 = st.as_bytes()[br_ind] as char;
                        match ch2 {
                            '(' => { br_count += 1 }
                            ')' => { br_count -= 1 }
                            '\r' => { break; } // end of the string
                            _ => {}
                        }
                    }
                    let str_part = st[(i + 1)..br_ind].to_string() + "\r\n"; // get the string part in brackets
                    i = br_ind;
                    first_sub = i + 1;

                    let new_fraction = make_operation(str_part); // get the result of the operation from brackets
                    fr_vec.push(new_fraction);
                    br_found = true;
                }
                '*' | '/' | '+' | '-' | '^' | '\r' => {
                    op_vec.push(ch);
                    if !br_found { // add a new fraction to the vector; if a bracket was found, it means it was already added to the vector
                        let mut str_part = st[first_sub..i].to_string();
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
    
        // do exponantation
        let mut ind: i32 = op_vec.len() as i32 - 1; // cannot be unsigned, otherwise it could break when decrementing when it's 0
        while ind >= 0 {
            let uind = ind as usize;
            match op_vec[uind] {
                '^' => {
                    let new_fr = Fraction::new_val(fr_vec[uind].value().powf(fr_vec[uind + 1].value()));
                    fr_vec[uind] = new_fr;
                    op_vec.remove(uind);
                    fr_vec.remove(uind + 1);
                }
                _ => { ind -= 1; }
            }
        }

        // do multiplication and division
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
    
        // do addition and subtraction
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

