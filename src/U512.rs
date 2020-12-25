use num_traits::{Num, One, Zero};
use std::ops::{Mul, Add, Rem, Div, Sub};
use num_integer::Integer;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
#[derive(Copy, Clone)]
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

    pub fn mulSlow(self, rhs: U512) -> U512 {
        let x = self.asArray();
        let y = rhs.asArray();

        let mut z: [u64; 8] = [0; 8];

        let mut carry_val: u128 = 0;
        for i in (0..8).rev() {
            for j in ((7 - i)..8).rev() {
                let k = (i + j - 7) as usize;
                carry_val = (x[i] as u128) * (y[j] as u128) + (z[k] as u128) + carry_val;

                z[k] = carry_val as u64;
                carry_val = carry_val >> 64;
            }
        }
        return fromArray(z);
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

impl Eq for U512 {
}

impl PartialOrd for U512 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl PartialEq for U512 {
    fn eq(&self, other: &Self) -> bool {
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
        let x = self.asArray();
        let y = rhs.asArray();

        let mut z1: [u64; 4] = [0; 4];
        let mut z2: [u64; 4] = [0; 4];
        let mut carry_val1: u128 = 0;
        let mut carry_val2: u128 = 0;
        for i in (0..4).rev() {
            for j in ((3 - i)..4).rev() {
                let k = (i + j - 3) as usize;

                carry_val1 = (x[i] as u128) * (y[j] as u128) + (z1[k] as u128) + carry_val1;
                z1[k] = carry_val1 as u64;
                carry_val1 = carry_val1 >> 64;

                carry_val2 = (x[i] as u128 - x[i + 4] as u128) * (y[j] as u128 - y[j + 4] as u128) + (z2[k] as u128) + carry_val2;
                z2[k] = carry_val2 as u64;
                carry_val2 = carry_val2 >> 64;

            }
        }

        let mut z3: [u64; 8] = [0; 8];
        let mut carry_val: u128 = 0;
        for i in (0..4).rev() {
            for j in (0..4).rev() {
                let k = (i + j + 1) as usize;
                carry_val = (x[i + 4] as u128) * (y[j + 4] as u128) + (z3[k] as u128) + carry_val;
                z3[k] = carry_val as u64;
                carry_val = carry_val >> 64;
            }
        }
        z3[0] = carry_val as u64;

        let mut z: [u64; 8] = [0; 8];
        let mut carry_val = 0;
        for i in (4..8).rev() {
            z[i] = z3[i];
        }
        for i in (0..4).rev() {
            carry_val = (z1[i] as u128) + (z3[i + 4] as u128) + (z3[i] as u128) + carry_val - (z2[i] as u128);
            z[i] = carry_val as u64;
            carry_val = carry_val >> 64;
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
