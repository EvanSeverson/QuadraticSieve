use num_traits::{Num, One, Zero};
use std::ops::{Mul, Add, Rem, Div, Sub};
use num_integer::Integer;
use std::cmp::Ordering;

struct U512 {
    first: u128,
    second: u128,
    third: u128,
    fourth: u128,
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

impl Mul<U512> for U512 {
    type Output = U512;

    fn mul(self, rhs: U512) -> Self::Output {
        let n64 = 1 << 64;
        
        let x1 = self.first >> 64;
        let x2 = self.first % n64;
        let x3 = self.second >> 64;
        let x4 = self.second % n64;
        let x5 = self.third >> 64;
        let x6 = self.third % n64;
        let x7 = self.fourth >> 64;
        let x8 = self.fourth % n64;

        let y1 = rhs.first >> 64;
        let y2 = rhs.first % n64;
        let y3 = rhs.second >> 64;
        let y4 = rhs.second % n64;
        let y5 = rhs.third >> 64;
        let y6 = rhs.third % n64;
        let y7 = rhs.fourth >> 64;
        let y8 = rhs.fourth % n64;

        let z11 = x1 * y8;
        let z12 = x2 * y7;
        let z13 = x3 * y6;
        let z14 = x4 * y5;
        let z15 = x5 * y4;
        let z16 = x6 * y3;
        let z17 = x7 * y2;
        let z18 = x8 * y1;

        let z21 = x1 * y7;
        let z22 = x2 * y6;
        let z23 = x3 * y5;
        let z24 = x4 * y4;
        let z25 = x5 * y3;
        let z26 = x6 * y2;
        let z27 = x7 * y1;

        let z31 = x1 * y6;
        let z32 = x2 * y5;
        let z33 = x3 * y4;
        let z34 = x4 * y3;
        let z35 = x5 * y2;
        let z36 = x6 * y1;

        let z41 = x1 * y5;
        let z42 = x2 * y4;
        let z43 = x3 * y3;
        let z44 = x4 * y2;
        let z45 = x5 * y1;

        let z51 = x1 * y4;
        let z52 = x2 * y3;
        let z53 = x3 * y2;
        let z54 = x4 * y1;

        let z61 = x1 * y3;
        let z62 = x2 * y2;
        let z63 = x3 * y1;

        let z71 = x1 * y2;
        let z72 = x2 * y1;

        let z81 = x1 * y1;

        let mut overflow = z81 / n64;
        let mut overflow2: u128 = 0;
        let z8 = z81 % n64;

        // let mut z7;

        overflow = 0;
        overflow2 = 0;

        return U512 { first: 0, second: 0, third: 0, fourth: 0 }

    }
}

impl Add<U512> for U512 {
    type Output = U512;

    fn add(self, rhs: U512) -> Self::Output {
        let (fourth, overflow4) = self.fourth.overflowing_add(rhs.fourth);

        let (third, mut overflow3) = self.third.overflowing_add(rhs.third);
        if overflow4 {
            let (third, tmp) = third.overflowing_add(1);
            overflow3 |= tmp;
        }

        let (second, mut overflow2) = self.second.overflowing_add(rhs.second);
        if overflow3 {
            let (second, tmp) = second.overflowing_add(1);
            overflow2 |= tmp;
        }

        let mut first = self.first + rhs.first;
        if overflow2 {
            first += 1;
        }
        return U512{first, second, third, fourth};
    }
}

impl Rem for U512 {
    type Output = U512;

    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Div for U512 {
    type Output = U512;

    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Sub for U512 {
    type Output = U512;

    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}