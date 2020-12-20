use std::io;
use std::str::FromStr;
use gmp::mpz::Mpz;
use num_integer::{Roots, Integer};
use std::time::SystemTime;
use primitive_types::U512;
use num_traits::{Num, One, Zero};

fn main() {
    println!("Please enter the number you wish to factor");

    // let mut val = String::new();
    // io::stdin()
    //     .read_line(&mut val)
    //     .expect("Failed to read line");
    // let val = &val.trim().parse().expect("Failed to parse");

    let vals = ["238305947612245937", "119886712242830033", "347904798577057303",
        "45211667757781417", "223302725014881931", "72216575918196841",
        "522112699961772923", "276590696103561107", "801139792682721823",
        "307443312955238791"];
    let now = SystemTime::now();
    for val in &vals {
        let val = U512::from_dec_str(&val.trim()).expect("Unable to parse");
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

fn findNontrivialFactor(val: &U512) -> U512 {
    let sqrt = val.sqrt();
    let mut a = sqrt + 1;
    while &a < &val {
        let b2 = (&a * &a) % val;
        let b = b2.sqrt();

        if &b * &b == b2 {
            let x = gcd(&val, &(&a - &b));
            if x != 1 {
                return x;
            }
        }
        a = &a + 1;
    }

    return U512::one();
}

fn gcd(a: &U512, b: &U512) -> U512 {
    let mut a = *a;
    let mut b = *b;

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }

    return a;
}
