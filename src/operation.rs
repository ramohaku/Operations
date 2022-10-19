pub mod fraction;
use crate::operation::fraction::fr::Fraction;

pub mod op {
    use super::fraction::fr::Fraction;

    pub fn make_operation(str : String) -> Fraction {
        let mut fr_vec: Vec<Fraction> = Vec::new();
        let mut op_vec: Vec<char> = Vec::new();
    
        let mut first_sub = 0;
        let mut br_found = false;
        let mut i = 0;
        while i < str.len() {
            let ch = str.as_bytes()[i] as char;
            match ch {
                '(' => {
                    let mut br_count = 1;
                    let mut br_ind = i;
                    while br_count > 0 {
                        br_ind += 1;
                        let ch2 = str.as_bytes()[br_ind] as char;
                        match ch2 {
                            '(' => { br_count += 1 }
                            ')' => { br_count -= 1 }
                            '\r' => { break; }
                            _ => {}
                        }
                    }
                    let str_part = str[(i + 1)..br_ind].to_string() + "\r\n";
                    i = br_ind;
                    first_sub = i + 1;

                    let new_fraction = make_operation(str_part);
                    fr_vec.push(new_fraction);
                    br_found = true;
                }
                '*' | '/' | '+' | '-' | '\r' => {
                    op_vec.push(ch);
                    if !br_found {
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
        fr_vec[0]
    }
    
}

