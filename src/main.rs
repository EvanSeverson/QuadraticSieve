use std::{io, mem};
use std::str::FromStr;
use num_bigint::{BigUint, ParseBigIntError};
use num_traits::One;
use std::mem::swap;

fn main() {
    println!("Please enter the number you wish to factor");

    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");
    let val: BigUint = match BigUint::from_str(&val.trim()) {
        Ok(s) => {s}
        Err(_) => {
            println!("Bad number.");
            return;
        }
    };

    let factor = findNontrivialFactor(&val);

    println!("a factor is {}", factor);
}

fn findNontrivialFactor(val: &BigUint) -> BigUint {
    let sqrt = val.sqrt();
    let mut a: BigUint = sqrt + BigUint::one();
    while &a < &val {
        let b2 = (&a * &a) % val;
        let b = b2.sqrt();

        if &b * &b == b2 {
            // let x = val.gcd(&(&a - &b));
            let x = gcd(val, &(&a - &b));
            if x != BigUint::one() {
                return x;
            }
        }
        a = &a + BigUint::one();
    }

    return BigUint::one();
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let zero: u8 = 0;
    let zero = &BigUint::from(zero);
    if a == zero {
        return b.clone();
    }
    if b == zero {
        return a.clone();
    }

    let mut m = a.clone();
    let mut n = b.clone();

    if m > n {
        swap(&mut m, &mut n);
    }


    while &m != zero {
        let tmp = &n % &m;
        n = m;
        m = tmp;
    }

    return n;
}
