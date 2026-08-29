mod consts;
pub use consts::Consts;

mod gcd;
use gcd::gdc_nonzerou128 as gcd;

mod numbers;

use core::num::NonZeroU128;

use crate::{Error, Expr};

#[macro_use]
mod macros;

pub(crate) use macros::rat;

#[inline(always)]
const fn apply_sign(num: i128, neg: bool) -> i128 {
    let mask = -(neg as i128);
    (num ^ mask) - mask
}

#[derive(Debug, Clone)]
pub struct Rational {
    //Numerator - Also stores the sign of the rational number
    num: i128,
    //Denominator
    den: NonZeroU128,
}

impl Rational {
    pub const ZERO: Rational = Rational {
        num: 0,
        // Safety: 1 is non-zero
        den: match NonZeroU128::new(1) {
            Some(v) => v,
            None => panic!("Is non-zero"),
        },
    };

    #[inline(always)]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    pub fn new(num: i128, den: u128) -> Result<Self, Error> {
        let den = NonZeroU128::new(den).ok_or(Error::DivisionByZero)?;
        if num == 0 {
            return Ok(Self::zero());
        }
        Ok(Self::const_reduce(Rational { num, den }))
    }

    pub const fn unwrap_new(num: i128, den: u128) -> Self {
        match NonZeroU128::new(den) {
            None => panic!("division by zero"),
            Some(den_nz) => {
                if num == 0 {
                    return Self::ZERO;
                }
                Self::const_reduce(Rational {
                    num: num,
                    den: den_nz,
                })
            }
        }
    }

    pub(crate) const fn const_reduce(mut r: Rational) -> Rational {
        if r.num == 0 {
            return Self::ZERO;
        };

        let num_nz = match NonZeroU128::new(r.abs_num()) {
            Some(v) => v,
            None => return Self::ZERO,
        };

        let div = gcd(num_nz, r.den);
        r.num = r.num.wrapping_div(div.get() as i128);
        r.den = match NonZeroU128::new(r.den.get() / div.get()) {
            Some(v) => v,
            None => panic!("den became zero after division by gcd — impossible"),
        };

        r
    }

    pub fn reduce_in_place(&mut self) -> &mut Self {
        *self = Self::const_reduce(std::mem::replace(self, Self::ZERO));
        self
    }

    pub fn reduce(&self) -> Self {
        let mut new = self.clone();
        new.reduce_in_place();
        new
    }

    pub fn checked_div(self, rhs: Self) -> Result<Self, Error> {
        if rhs.num == 0 {
            Err(Error::DivisionByZero)
        } else {
            let num_mag = self.abs_num() * rhs.den.get();
            let den = self.den.get() * rhs.abs_num();
            let neg = self.is_neg() ^ rhs.is_neg();

            let num: i128 = num_mag.try_into().map_err(|_| Error::Overflow)?;
            Ok(Self::new(apply_sign(num, neg), den)?)
        }
    }

    pub fn pow(&self, other: &Rational) -> Result<Expr, Error> {
        if !other.is_integer() {
            return Err(Error::RootsNotImplemented);
        }
        let pow: u32 = other.abs_num().try_into().map_err(|_| Error::Overflow)?;

        let new_num = self.abs_num().checked_pow(pow).ok_or(Error::Overflow)?;
        let new_den = self.den.get().checked_pow(pow).ok_or(Error::Overflow)?;

        //Check if self is neg and pow is not even
        let new_neg = self.is_neg() && !(pow % 2 == 0);

        let (new_num, new_den) = if other.is_neg() {
            (new_den, new_num)
        } else {
            (new_num, new_den)
        };

        let num = apply_sign(new_num.try_into().map_err(|_| Error::Overflow)?, new_neg);
        Rational::new(num, new_den).map(|r| r.into())
    }

    #[inline]
    pub const fn const_neg(self) -> Self {
        Rational {
            num: -self.num,
            den: self.den,
        }
    }

    #[inline(always)]
    pub const fn is_integer(&self) -> bool {
        self.den.get() == 1
    }

    #[inline(always)]
    pub const fn is_neg(&self) -> bool {
        self.num < 0
    }

    #[inline(always)]
    pub const fn abs_num(&self) -> u128 {
        self.num.unsigned_abs()
    }
}

impl Default for Rational {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.den.get() {
            1 => write!(f, "{}", self.num),
            den => write!(f, "{}/{}", self.num, den),
        }
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        // Since both den are non-zero, this new is also non-zero
        let den = self.den.get() * rhs.den.get();

        //Safety: den is non-zero
        Self::new(num, den).unwrap()
    }
}

impl std::ops::Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let left_num = self.num * (rhs.den.get() as i128);
        let right_num = rhs.num * (self.den.get() as i128);

        let den = self.den.get() * rhs.den.get();

        //Safety: den is non-zero
        Self::new(left_num + right_num, den).unwrap()
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Neg for &Rational {
    type Output = Rational;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.clone().const_neg()
    }
}

impl std::ops::Neg for Rational {
    type Output = Self;
    #[inline(always)]
    fn neg(mut self) -> Self::Output {
        self.const_neg()
    }
}

impl std::ops::Div for Rational {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).expect("division by zero")
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        (self.num == 0 && other.num == 0) || {
            let a = self.reduce();
            let b = other.reduce();
            a.num == b.num && a.den == b.den
        }
    }
}

impl_ops!(Add, add);
impl_ops!(Sub, sub);
impl_ops!(Mul, mul);
impl_ops!(Div, div);

#[cfg(test)]
mod tests {
    use super::Rational;

    #[test]
    fn eq_test() {
        assert_eq!(Rational::zero(), Rational::zero());
        assert_eq!(rat!(2 / 3), rat!(4 / 6));
        assert_eq!(rat!(0), rat!(0));
        assert_ne!(rat!(-2 / 3), rat!(2 / 3));
        assert_ne!(rat!(-4 / 3), rat!(8 / 6));
        assert_ne!(rat!(-0 / 100), rat!(8 / 2));
        assert_ne!(rat!(0 / 1000), rat!(8 / 2));
        assert_eq!(rat!(0 / 1000), rat!(-0));
    }

    #[test]
    fn neg_test() {
        assert_eq!(rat!(0), -rat!(0));
        assert_eq!(rat!(-1), -rat!(1));
        assert_eq!(Rational::unwrap_new(-1, 1), -Rational::unwrap_new(1, 1),);
        assert_ne!(Rational::unwrap_new(-1, 1), -Rational::unwrap_new(-1, 1));
        assert_eq!({ -Rational::zero() }.is_neg(), false);
    }

    #[test]
    fn add_test() {
        assert_eq!(rat!(1) + rat!(1), rat!(2));
        assert_eq!(rat!(1 / 2) + rat!(1 / 2), rat!(1));
        assert_eq!(rat!(-1 / 2) + rat!(1 / 2), rat!(0));
        assert_eq!(rat!(99 / 100) + rat!(1 / 100), rat!(1));
        assert_eq!(rat!(3 / 5) + rat!(7 / 11), rat!(68 / 55));
        assert_eq!(rat!(3 / 5) + rat!(-7 / 11), rat!(-2 / 55));
        assert_eq!(rat!(-7 / 11) + rat!(3 / 5), rat!(-2 / 55));
        assert_eq!(rat!(-2) + rat!(-2), rat!(-4));
    }

    #[test]
    fn reduce_test() {
        //Rational::new automatically reduces
        assert_eq!(rat!(0).reduce(), rat!(0));
        assert_eq!(rat!(2 / 4), rat!(1 / 2));
        assert_eq!(rat!(2 / 4), rat!(1 / 2));
        assert_eq!(rat!(100 / 50), rat!(8 / 4));
        assert_eq!(rat!(-100 / 50), -rat!(8 / 4));
    }

    #[test]
    fn mul_test() {
        assert_eq!(rat!(0) * rat!(0), rat!(0));
        assert_eq!(rat!(2) * rat!(4), rat!(8));
        assert_eq!(rat!(-2) * rat!(1 / 2), rat!(-1));
        assert_eq!(rat!(-1) * rat!(-1), rat!(1));
    }

    #[test]
    fn div_test() {
        rat!(1)
            .checked_div(rat!(0))
            .expect_err("Should be division by zero");

        assert_eq!(rat!(4) / rat!(2), rat!(2));
        assert_eq!(rat!(7 / 3) / rat!(7 / 3), rat!(1));
        assert_eq!(rat!(7 / 3) / rat!(-7 / 3), rat!(-1));
        assert_eq!(rat!(4 / 2) / rat!(-2 / 4), rat!(-16 / 4));
    }

    #[test]
    fn sub_test() {
        assert_eq!(rat!(1) - rat!(1), rat!(0));
        assert_eq!(rat!(1 / 2) - rat!(1 / 2), rat!(0));
        assert_eq!(rat!(-1 / 2) - rat!(1 / 2), rat!(-1));
        assert_eq!(rat!(99 / 100) - rat!(1 / 100), rat!(98 / 100));
        assert_eq!(rat!(-2) - rat!(-2), rat!(0));
    }

    #[test]
    fn create_test() {
        assert_eq!(Rational::new(-0, 1).unwrap(), rat!(0));
        Rational::new(0, 0).expect_err("Should be division by zero");
    }

    #[test]
    fn is_test() {
        assert_eq!(rat!(1).is_neg(), false);
        assert_eq!(rat!(-1).is_neg(), true);
        assert_eq!(rat!(1 / 3).is_neg(), false);
        assert_eq!(rat!(-1 / 3).is_neg(), true);

        assert_eq!(rat!(1).is_integer(), true);
        assert_eq!(rat!(2).is_integer(), true);
        assert_eq!(rat!(3).is_integer(), true);
        assert_eq!(rat!(-3).is_integer(), true);
        assert_eq!(rat!(3 / 2).is_integer(), false);
        assert_eq!(rat!(1 / 3).is_integer(), false);
        assert_eq!(rat!(4 / 12).is_integer(), false);
    }
}

#[cfg(test)]
mod apply_sign_test {
    use super::apply_sign;

    #[test]
    fn positive_to_positive() {
        assert_eq!(apply_sign(0, false), 0);
        assert_eq!(apply_sign(1, false), 1);
        assert_eq!(apply_sign(123, false), 123);
        assert_eq!(apply_sign(i128::MAX, false), i128::MAX);
    }

    #[test]
    fn negative_to_negative() {
        assert_eq!(apply_sign(0, true), 0);
        assert_eq!(apply_sign(1, true), -1);
        assert_eq!(apply_sign(123, true), -123);
        assert_eq!(apply_sign(i128::MAX, true), -i128::MAX);
    }

    #[test]
    fn zero_stays() {
        assert_eq!(apply_sign(0, false), 0);
        assert_eq!(apply_sign(0, true), 0);
        assert_eq!(apply_sign(0, true), apply_sign(0, false));
    }
}
