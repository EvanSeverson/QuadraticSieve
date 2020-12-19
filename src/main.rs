use std::io;
use std::str::FromStr;
use gmp::mpz::Mpz;

fn main() {
    println!("Please enter the number you wish to factor");

    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");
    let val = match gmp::mpz::Mpz::from_str(&val) {
        Ok(s) => {s}
        Err(_) => {
            println!("Bad number.");
            return;
        }
    };
    let factor = findNontrivialFactor(&val);

    println!("a factor is {}", factor);
}

fn findNontrivialFactor(val: &Mpz) -> Mpz {
    let sqrt = val.sqrt();
    let mut a = sqrt + 1;
    while &a < &val {
        let b2 = (&a * &a) % val;
        let b = b2.sqrt();

        if &b * &b == b2 {
            let x = val.gcd(&(&a - &b));
            if x != Mpz::one() {
                return x;
            }
        }
        a = &a + 1;
    }

    return Mpz::one();
}
