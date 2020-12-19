use std::io;
use std::str::FromStr;
use gmp::mpz::Mpz;
use std::time::SystemTime;

fn main() {

    // let mut val = String::new();
    // io::stdin()
    //     .read_line(&mut val)
    //     .expect("Failed to read line");
    // let val = match gmp::mpz::Mpz::from_str(&val) {
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
        let val = match gmp::mpz::Mpz::from_str(&val) {
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
