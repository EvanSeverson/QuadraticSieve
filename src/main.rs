use std::{io, mem};
use std::str::FromStr;
use num_bigint::{BigUint, ParseBigIntError};
use num_traits::One;
use std::mem::swap;
use std::time::SystemTime;

fn main() {
    // println!("Please enter the number you wish to factor");

    // let mut val = String::new();
    // io::stdin()
    //     .read_line(&mut val)
    //     .expect("Failed to read line");
    // let val: BigUint = match BigUint::from_str(&val.trim()) {
    //     Ok(s) => {s}
    //     Err(_) => {
    //         println!("Bad number.");
    //         return;
    //     }
    // };

    let vals = ["238305947612245937", "119886712242830033", "347904798577057303",
        "45211667757781417", "223302725014881931", "72216575918196841",
        "522112699961772923", "276590696103561107", "801139792682721823",
        "307443312955238791"];

    let now = SystemTime::now();

    for val in &vals {
        let val: BigUint = match BigUint::from_str(&val.trim()) {
            Ok(s) => {s}
            Err(_) => {
                println!("Bad number.");
                return;
            }
        };
        let factor = findNontrivialFactor(&val);

        println!("a factor for {} is {}", val, factor);
    }




    match now.elapsed() {
        Ok(elapsed) => {
            println!("took {} micros", elapsed.as_micros());}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
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
