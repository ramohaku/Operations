pub mod fr {
    use std::ops::{self, Mul, Div, MulAssign};
    
    pub type FrI = i64; // int type for numenator and denominator
    pub type FrF = f64; // float type for the real value

    // gcd algorithm (works also for negative numbers)
    fn gcd(mut a: FrI, mut b: FrI) -> FrI {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Fraction {
        pub num: FrI, // numenator
        pub den: FrI,  // denomiantor
    }

    impl Fraction {
        // basic constructor
        pub fn new(num: FrI, den: FrI) -> Self {
            Fraction { num, den }.shorten()
        }
        
        // constructor for the real value from string
        pub fn new_val_str(st : &mut String) -> Self {
            let point = st.find('.');//.unwrap();
            if point == None {
                let try_num = st.parse::<FrI>();
                let mut num : FrI = 0;
                match try_num {
                    Ok(val) => { num = val },
                    Err(e) => {
                        println!("Invalid characters!");
                    }
                }
                return Fraction { num, den: 1 };
            }
    
            let pos = point.unwrap();
    
            let power = (st.len() - pos - 1) as u32;
            let mut st = st.replace(".", "");
    
            let num = st.parse::<FrI>().unwrap();
            let base: FrI = 10;
            let den = base.pow(power);
    
            return Fraction { num, den }.shorten();
        }
    
        // constructor for getting the real value
        pub fn new_val(v : FrF) -> Self {
            let mut n = v.to_string();
    
            Self::new_val_str(&mut n)
        }
    
        // returns the real value
        pub fn value(&self) -> FrF {
            self.num as FrF / self.den as FrF
        }
    
        // shortens the fraction
        pub fn shorten(&mut self) -> Self {
            let gcd_div = gcd(self.num, self.den);
            self.num /= gcd_div;
            self.den /= gcd_div;
    
            *self
        }
    }
    
    ///////////////////// operators ////////////////////////

    impl ops::Mul for Fraction {
        type Output = Self;
    
        fn mul(self, rhs: Self) -> Self {
            let mut fr1 : Fraction = Fraction::new(self.num, rhs.den);
            let mut fr2 : Fraction = Fraction::new(rhs.num, self.den);
    
            Fraction { num: fr1.num * fr2.num, den: fr1.den * fr2.den }
        }
    }
    
    impl ops::MulAssign for Fraction {
        fn mul_assign(&mut self, rhs: Self) {
            let mut fr1 : Fraction = Fraction::new(self.num, rhs.den);
            let mut fr2 : Fraction = Fraction::new(rhs.num, self.den);
    
            self.num = fr1.num * fr2.num;
            self.den = fr1.den * fr2.den;
        }
    }
    
    impl ops::Div for Fraction {
        type Output = Fraction;
    
        fn div(self, rhs: Self) -> Self {
            self.mul(Fraction { num: rhs.den, den: rhs.num })
        }
    }
    
    impl ops::DivAssign for Fraction {
        fn div_assign(&mut self, rhs: Self) {
            self.mul_assign(Fraction { num: rhs.den, den: rhs.num });
        }
    }
    
    // struct for adding or substracting two fractions
    struct ComDenFr {
        num1: FrI,
        num2: FrI,
        com_den: FrI
    }
    
    // gets ComDenFr from two fractions
    fn get_cdf(fr1: Fraction, fr2: Fraction) -> ComDenFr {
        let com_den = fr1.den * fr2.den / gcd(fr1.den, fr2.den);
        let num1 = com_den / fr1.den * fr1.num;
        let num2 = com_den / fr2.den * fr2.num;
    
        ComDenFr { num1, num2, com_den }
    }
    
    impl ops::Add for Fraction {
        type Output = Fraction;
    
        fn add(self, rhs: Self) -> Self {
            let cdf = get_cdf(self, rhs);
    
            Fraction::new(cdf.num1 + cdf.num2, cdf.com_den)
        }
    }
    
    impl ops::AddAssign for Fraction {
        fn add_assign(&mut self, rhs: Self) {
            let cdf = get_cdf(*self, rhs);
    
            self.num = cdf.num1 + cdf.num2;
            self.den = cdf.com_den;
            self.shorten();
        }
    }
    
    impl ops::Sub for Fraction {
        type Output = Fraction;
    
        fn sub(self, rhs: Self) -> Self {
            let cdf = get_cdf(self, rhs);
    
            Fraction::new(cdf.num1 - cdf.num2, cdf.com_den)
        }
    }
    
    impl ops::SubAssign for Fraction {
        fn sub_assign(&mut self, rhs: Self) {
            let cdf = get_cdf(*self, rhs);
    
            self.num = cdf.num1 - cdf.num2;
            self.den = cdf.com_den;
            self.shorten();
        }
    }
}