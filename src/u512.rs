use num_traits::{Num, One, Zero};
use std::ops::{Mul, Add, Rem, Div, Sub, Shl, Shr};
use num_integer::Integer;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(PartialOrd)]
pub struct U512 {
    data: [u64; 8]
}

impl U512 {
    pub fn as_array(&self) -> [u64; 8] {
        return self.data;
    }

    pub fn from_array(arr: [u64; 8]) -> U512 {
        return U512{data: arr};
    }

    pub fn from(x: u128) -> U512 {
        return U512{data: [x as u64, (x >> 64) as u64, 0, 0, 0, 0, 0, 0, ]}
    }
}

pub fn from_array(arr: [u64; 8]) -> U512 {
    return U512{data: arr}
}

pub fn from(x: u128) -> U512 {
    return U512{data: [x as u64, (x >> 64) as u64, 0, 0, 0, 0, 0, 0, ]}
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

impl Add for U512 {
    type Output = U512;

    fn add(self, rhs: U512) -> Self::Output {
        let x = self.as_array();
        let y = rhs.as_array();

        let mut z: [u64; 8] = [0; 8];
        
        let mut carry_val: u64 = 0;
        for i in 0..8 {
            let (zi, carry) = x[i].overflowing_add(y[i]);
            let (zi, carry2) = zi.overflowing_add(carry_val);
            z[i] = zi;
            carry_val = match carry || carry2 {
                true => 1,
                false => 0,
            };
        }
        
        return from_array(z);
    }
}

impl Add<u128> for U512 {
    type Output = U512;

    fn add(self, rhs: u128) -> Self::Output {
        return self + from(rhs);
    }
}

impl Add<i128> for U512 {
    type Output = U512;

    fn add(self, rhs: i128) -> Self::Output {
        if rhs < 0 {
            return self - ((-1 * rhs) as u128);
        }
        return self + (rhs as u128);
    }
}

macro_rules! unsigned_add {
    ($T:ident) => {
        impl Add<$T> for U512 {
            type Output = U512;

            fn add(self, rhs: $T) -> Self::Output {
                let x = self.as_array();
                let rhs = rhs as u64;

                let (z0, carry) = x[0].overflowing_add(rhs);
                let mut carry_val = match carry {
                    true => 1,
                    false => 0,
                };
                let mut z = [0u64; 8];
                z[0] = z0;

                for i in 1..8 {
                    if (carry_val == 0) {
                        return from_array(z);
                    }
                    let (zi, carry) = x[i].overflowing_add(carry_val);
                    z[i] = zi;
                    carry_val = match carry {
                        true => 1,
                        false => 0,
                    };
                }
                return from_array(z);
            }
        }
    }
}

unsigned_add!(u8);
unsigned_add!(u16);
unsigned_add!(u32);
unsigned_add!(u64);

macro_rules! signed_add {
    ($T:ident) => {
        impl Add<$T> for U512 {
            type Output = U512;

            fn add(self, rhs: $T) -> Self::Output {
                if (rhs > 0) {
                    return self + (rhs as u64);
                }
                return self + (-rhs as u64);
            }
        }
    }
}

signed_add!(i8);
signed_add!(i16);
signed_add!(i32);
signed_add!(i64);

impl Mul for U512 {
    type Output = U512;

    fn mul(self, rhs: U512) -> Self::Output {
        let zero = U512::from(0);
        if self == zero || rhs == zero {
            return zero;
        }
        let x = self.as_array();
        let y = rhs.as_array();

        let mut z: [u64; 8] = [0; 8];

        let mut carry_val: u128 = 0;
        for i in 0..8 {
            for j in 0..(8 - i) {
                let k = i + j;
                carry_val = (x[i] as u128) * (y[j] as u128) + (z[k] as u128) + carry_val;

                z[k] = carry_val as u64;
                carry_val = carry_val >> 64;
            }
            carry_val = 0;
        }
        return from_array(z);
    }
}

impl Mul<u128> for U512 {
    type Output = U512;

    fn mul(self, rhs: u128) -> Self::Output {
        return self * from(rhs);
    }
}

macro_rules! unsigned_mul {
    ($T:ident) => {
        impl Mul<$T> for U512 {
            type Output = U512;

            fn mul(self, rhs: $T) -> Self::Output {
                let zero = U512::from(0);
                if self == zero || rhs == 0 {
                    return zero;
                }
                let y = rhs as u128;
                let x = self.as_array();

                let mut z: [u64; 8] = [0; 8];

                let mut carry_val: u128 = 0;
                for i in 0..8 {
                    carry_val = (x[i] as u128) * y + carry_val;

                    z[i] = carry_val as u64;
                    carry_val = carry_val >> 64;
                }
                return from_array(z);
            }
        }
    }
}

unsigned_mul!(u8);
unsigned_mul!(u16);
unsigned_mul!(u32);
unsigned_mul!(u64);

macro_rules! signed_mul {
    ($T:ident) => {
        impl Mul<$T> for U512 {
            type Output = U512;

            fn mul(self, rhs: $T) -> Self::Output {
                if rhs >= 0 {
                    return self * (rhs as u64);
                }
                panic!("Don't try to turn this unsigned integer negative :("); // TODO although it
                // shouldn't make sense to negate an unsigned integer I want this to be defined as
                // the value mod 2^512
            }
        }
    }
}

signed_mul!(i8);
signed_mul!(i16);
signed_mul!(i32);
signed_mul!(i64);
signed_mul!(i128);

impl Sub for U512 {
    type Output = U512;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.as_array();
        let y = rhs.as_array();

        let mut z: [u64; 8] = [0; 8];

        let mut carry_val: u64 = 0;
        for i in 0..8 {
            let (zi, borrow) = x[i].overflowing_sub(y[i]);
            let (zi, borrow2) = zi.overflowing_sub(carry_val);
            z[i] = zi;
            carry_val = match borrow || borrow2 {
                true => 1,
                false => 0,
            };
        }

        return from_array(z);
    }
}

impl Sub<u128> for U512 {
    type Output = U512;

    fn sub(self, rhs: u128) -> Self::Output {
        return self - from(rhs);
    }
}

impl Sub<i128> for U512 {
    type Output = U512;

    fn sub(self, rhs: i128) -> Self::Output {
        if rhs < 0 {
            return self + ((-1 * rhs) as u128);
        }
        return self + (rhs as u128);
    }
}

macro_rules! unsigned_sub {
    ($T:ident) => {
        impl Sub<$T> for U512 {
            type Output = U512;

            fn sub(self, rhs: $T) -> Self::Output {
                let x = self.as_array();
                let rhs = rhs as u64;

                let (z0, borrow) = x[0].overflowing_sub(rhs);
                let mut borrow_val = match borrow {
                    true => 1,
                    false => 0,
                };
                let mut z = [0u64; 8];
                z[0] = z0;

                for i in 1..8 {
                    if (borrow_val == 0) {
                        return from_array(z);
                    }
                    let (zi, borrow) = x[i].overflowing_sub(borrow_val);
                    z[i] = zi;
                    borrow_val = match borrow {
                        true => 1,
                        false => 0,
                    };
                }
                return from_array(z);
            }
        }
    }
}

unsigned_sub!(u8);
unsigned_sub!(u16);
unsigned_sub!(u32);
unsigned_sub!(u64);

macro_rules! signed_sub {
    ($T:ident) => {
        impl Sub<$T> for U512 {
            type Output = U512;

            fn sub(self, rhs: $T) -> Self::Output {
                if (rhs > 0) {
                    return self - (rhs as u64);
                }
                return self - ((- (rhs as i128)) as u64);
            }
        }
    }
}

signed_sub!(i8);
signed_sub!(i16);
signed_sub!(i32);
signed_sub!(i64);

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

macro_rules! shl {
    ($T:ident) => {
        impl Shl<$T> for U512 {
            type Output = U512;

            fn shl(self, rhs: $T) -> Self::Output {
                let mut arr = [0u64; 8];
                let datum_shift = (rhs / 64) as usize;
                let bit_shift = rhs % 64;

                arr[datum_shift] = self.data[0] << bit_shift;
                for i in (datum_shift + 1)..8 {
                    arr[i] = (self.data[i - datum_shift] << bit_shift) + (self.data[i - datum_shift - 1] >> (64 - bit_shift));
                }

                return from_array(arr);
            }
        }
    }
}

macro_rules! signed_shl {
    ($T:ident) => {
        impl Shl<$T> for U512 {
            type Output = U512;

            fn shl(self, rhs: $T) -> Self::Output {
                if rhs < 0 {
                    panic!("Can't shift by a negative number");
                }
                return self << (rhs as u128);
            }
        }
    }
}

shl!(u8);
shl!(u16);
shl!(u32);
shl!(u64);
shl!(u128);
signed_shl!(i8);
signed_shl!(i16);
signed_shl!(i32);
signed_shl!(i64);
signed_shl!(i128);

macro_rules! shr {
    ($T:ident) => {
        impl Shr<$T> for U512 {
            type Output = U512;

            fn shr(self, rhs: $T) -> Self::Output {
                let mut arr = [0u64; 8];
                let datum_shift = (rhs / 64) as usize;
                let bit_shift = rhs % 64;

                arr[datum_shift] = self.data[0] << bit_shift;
                for i in 0..(8 - datum_shift - 1) {
                    arr[i] = (self.data[i + datum_shift] >> bit_shift) + (self.data[i + datum_shift + 1] << (64 - bit_shift));
                }
                arr[8 - datum_shift - 1] = self.data[7] >> bit_shift;

                return from_array(arr);
            }
        }
    }
}

macro_rules! signed_shr {
    ($T:ident) => {
        impl Shr<$T> for U512 {
            type Output = U512;

            fn shr(self, rhs: $T) -> Self::Output {
                if rhs < 0 {
                    panic!("Can't shift by a negative number");
                }
                return self >> (rhs as u128);
            }
        }
    }
}

shr!(u8);
shr!(u16);
shr!(u32);
shr!(u64);
shr!(u128);
signed_shr!(i8);
signed_shr!(i16);
signed_shr!(i32);
signed_shr!(i64);
signed_shr!(i128);

impl Display for U512 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::u512::U512;

    #[test]
    fn test_addition_u128() {
        let mut a: u128 = 2;
        let mut b: u128 = 3;
        for _ in 1..1000000 {
            let a2 = U512::from(a);
            let b2 = U512::from(b);

            let c = a + b;
            let c2 = a2 + b2;

            assert_eq!(c as u64, c2.data[0]);
            assert_eq!((c >> 64) as u64, c2.data[1]);

            a = a.wrapping_mul(a).wrapping_add(1) % (1u128 << 63);
            b = b.wrapping_mul(b).wrapping_add(1) % (1u128 << 63);
        }
    }

    #[test]
    fn test_subtraction_u128() {
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

            assert_eq!(c as u64, c2.data[0]);
            assert_eq!((c >> 64) as u64, c2.data[1]);

            a = a.wrapping_mul(a).wrapping_add(1) % (1u128 << 63);
            b = b.wrapping_mul(b).wrapping_add(1) % (1u128 << 63);
        }
    }

    #[test]
    fn test_addition_subtraction_inverse() {
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
    fn test_addition_commutative() {
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
    fn test_multiplication_vs_binary_addition() {
        let mut a = U512::from(2);
        let mut b = U512::from(3);

        for _ in 1..20000 {
            let c = a * b;

            let mut c2 = U512::from(0);
            let mut a2 = a;
            let mut b2 = b;
            while b2 > U512::from(0) {
                if b2.data[0] % 2 == 1 {
                    c2 = c2 + a2;
                }
                // a2 = a2 + a2;
                a2 = a2 << 1;
                b2 = b2 >> 1;
            }

            assert_eq!(c, c2, "Testing multiplying {} and {}", a, b);

            a = a * a + U512::from(1);
            b = b * b + U512::from(1);
        }
    }

    #[test]
    fn test_multiplication_commutative() {
        let mut a = U512::from(2);
        let mut b = U512::from(3);

        for _ in 1..1000000 {

            assert_eq!(a * b, b * a);

            a = a * a + U512::from(1);
            b = b * b + U512::from(1);
        }
    }
}
