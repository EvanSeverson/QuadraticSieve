use num_traits::{Num, One, Zero};
use std::ops::{Mul, Add, Rem, Div, Sub};
use num_integer::Integer;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(PartialOrd)]
pub struct U512 {
    x7: u64,
    x6: u64,
    x5: u64,
    x4: u64,
    x3: u64,
    x2: u64,
    x1: u64,
    x0: u64,
}

impl U512 {
    pub fn asArray(&self) -> [u64; 8] {
        return [self.x7, self.x6, self.x5, self.x4, self.x3, self.x2, self.x1, self.x0, ];
    }

    pub fn fromArray(arr: [u64; 8]) -> U512 {
        return U512{
            x7: arr[0],
            x6: arr[1],
            x5: arr[2],
            x4: arr[3],
            x3: arr[4],
            x2: arr[5],
            x1: arr[6],
            x0: arr[7],
        }
    }

    pub fn from(x: u128) -> U512 {
        return U512{
            x7: 0,
            x6: 0,
            x5: 0,
            x4: 0,
            x3: 0,
            x2: 0,
            x1: (x >> 64) as u64,
            x0: x as u64,
        }
    }
}

pub fn fromArray(arr: [u64; 8]) -> U512 {
    return U512{
        x7: arr[0],
        x6: arr[1],
        x5: arr[2],
        x4: arr[3],
        x3: arr[4],
        x2: arr[5],
        x1: arr[6],
        x0: arr[7],
    }
}

pub fn from(x: u128) -> U512 {
    return U512{
        x7: 0,
        x6: 0,
        x5: 0,
        x4: 0,
        x3: 0,
        x2: 0,
        x1: (x >> 64) as u64,
        x0: x as u64,
    }
}

impl Integer for U512 {
    fn div_floor(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn gcd(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn lcm(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn divides(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn is_even(&self) -> bool {
        unimplemented!()
    }

    fn is_odd(&self) -> bool {
        unimplemented!()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        unimplemented!()
    }
}

impl Ord for U512 {
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}

impl Num for U512 {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl One for U512 {
    fn one() -> Self {
        unimplemented!()
    }
}

impl Zero for U512 {
    fn zero() -> Self {
        unimplemented!()
    }

    fn is_zero(&self) -> bool {
        unimplemented!()
    }
}

impl Add<U512> for U512 {
    type Output = U512;

    fn add(self, rhs: U512) -> Self::Output {
        let x = self.asArray();
        let y = rhs.asArray();

        let mut z: [u64; 8] = [0; 8];
        
        let mut carry_val: u64 = 0;
        for i in (0..8).rev() {
            let (zi, carry) = x[i].overflowing_add(y[i]);
            let (zi, carry2) = zi.overflowing_add(carry_val);
            z[i] = zi;
            carry_val = match carry || carry2 {
                true => 1,
                false => 0,
            };
        }
        
        return fromArray(z);
    }
}

impl Mul<U512> for U512 {
    type Output = U512;

    fn mul(self, rhs: U512) -> Self::Output {
        let zero = U512::from(0);
        if self == zero || rhs == zero {
            return zero;
        }
        let x = self.asArray();
        let y = rhs.asArray();

        let mut z: [u64; 8] = [0; 8];

        let mut carry_val: u128 = 0;
        for i in (0..8).rev() {
            for j in ((7 - i)..8).rev() {
                let k = i + j - 7;
                carry_val = (x[i] as u128) * (y[j] as u128) + (z[k] as u128) + carry_val;

                z[k] = carry_val as u64;
                carry_val = carry_val >> 64;
            }
            carry_val = 0;
        }
        return fromArray(z);
    }
}

impl Sub for U512 {
    type Output = U512;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.asArray();
        let y = rhs.asArray();

        let mut z: [u64; 8] = [0; 8];

        let mut carry_val: u64 = 0;
        for i in (0..8).rev() {
            let (zi, borrow) = x[i].overflowing_sub(y[i]);
            let (zi, borrow2) = zi.overflowing_sub(carry_val);
            z[i] = zi;
            carry_val = match borrow || borrow2 {
                true => 1,
                false => 0,
            };
        }

        return fromArray(z);
    }
}

impl Div for U512 {
    type Output = U512;

    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Rem for U512 {
    type Output = U512;

    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Display for U512 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{},{},{},{},{},{},{},{}",
               self.x7, self.x6, self.x5, self.x4, self.x3, self.x2, self.x1, self.x0)
    }
}

#[cfg(test)]
mod tests {
    use crate::U512::{U512, from};

    #[test]
    fn testAdditionU128() {
        let mut a: u128 = 2;
        let mut b: u128 = 3;
        for _ in 1..1000000 {
            let a2 = U512::from(a);
            let b2 = U512::from(b);

            let c = a + b;
            let c2 = a2 + b2;

            assert_eq!(c as u64, c2.x0);
            assert_eq!((c >> 64) as u64, c2.x1);

            a = a.wrapping_mul(a).wrapping_add(1) % (1u128 << 63);
            b = b.wrapping_mul(b).wrapping_add(1) % (1u128 << 63);
        }
    }

    #[test]
    fn testSubtractionU128() {
        let mut a: u128 = 2;
        let mut b: u128 = 3;
        for _ in 1..1000000 {
            if b > a {
                let tmp = a;
                a = b;
                b = tmp;
            }
            let a2 = U512::from(a);
            let b2 = U512::from(b);

            let c = a - b;
            let c2 = a2 - b2;

            assert_eq!(c as u64, c2.x0);
            assert_eq!((c >> 64) as u64, c2.x1);

            a = a.wrapping_mul(a).wrapping_add(1) % (1u128 << 63);
            b = b.wrapping_mul(b).wrapping_add(1) % (1u128 << 63);
        }
    }

    #[test]
    fn testAdditionSubtractionInverse() {
        let mut a = U512::from(2);
        let mut b = U512::from(3);

        for _ in 1..1000000 {
            let c = a + b;
            let a2 = c - b;
            let b2 = c - a;

            assert_eq!(a, a2);
            assert_eq!(b, b2);

            a = a * a + U512::from(1);
            b = b * b + U512::from(1);
        }
    }

    #[test]
    fn testAdditionCommutative() {
        let mut a = U512::from(2);
        let mut b = U512::from(3);

        for _ in 1..1000000 {

            assert_eq!(a + b, b + a);

            a = a * a + U512::from(1);
            b = b * b + U512::from(1);
        }
    }

    // This test actually found a bug that existed since the beginning. Binary doubling, analogous
    // to the binary squaring algorithm for exponentiation, isn't a very optimal algorithm but if
    // the test is run with --release then it runs about 60x faster so you can increase the number
    // of test cases.
    #[test]
    fn testMultiplicationVsBinaryAddition() {
        let mut a = U512::from(2);
        let mut b = U512::from(3);

        for _ in 1..20000 {
            let c = a * b;

            let mut c2 = U512::from(0);
            let mut a2 = a;
            let mut b2 = b;
            while b2 > U512::from(0) {
                if b2.x0 % 2 == 1 {
                    c2 = c2 + a2;
                }
                // a2 = a2 + a2;
                a2 = U512::fromArray([
                    (a2.x7 << 1) + (a2.x6 >> 63),
                    (a2.x6 << 1) + (a2.x5 >> 63),
                    (a2.x5 << 1) + (a2.x4 >> 63),
                    (a2.x4 << 1) + (a2.x3 >> 63),
                    (a2.x3 << 1) + (a2.x2 >> 63),
                    (a2.x2 << 1) + (a2.x1 >> 63),
                    (a2.x1 << 1) + (a2.x0 >> 63),
                    (a2.x0 << 1),
                ]);
                b2 = U512::fromArray([
                    (b2.x7 >> 1),
                    (b2.x6 >> 1) + ((b2.x7 % 2) << 63),
                    (b2.x5 >> 1) + ((b2.x6 % 2) << 63),
                    (b2.x4 >> 1) + ((b2.x5 % 2) << 63),
                    (b2.x3 >> 1) + ((b2.x4 % 2) << 63),
                    (b2.x2 >> 1) + ((b2.x3 % 2) << 63),
                    (b2.x1 >> 1) + ((b2.x2 % 2) << 63),
                    (b2.x0 >> 1) + ((b2.x1 % 2) << 63),
                ]);
            }

            assert_eq!(c, c2, "Testing multiplying {} and {}", a, b);

            a = a * a + U512::from(1);
            b = b * b + U512::from(1);
        }
    }
}
