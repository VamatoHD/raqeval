mod gcd;
use gcd::gdc_nonzerou128 as gcd;

mod unsigned;
use unsigned::Unsigned;

use core::num::NonZeroU128;

macro_rules! Ratio {
    (0 / $b:literal) => {{ Rational::zero(false) }};
    (- 0 / $b:literal) => {{ Rational::zero(true) }};
    ($a:literal / 0) => {{ compile_error!("Division by zero.") }};
    () => {{ compile_error!("Invalid Ratio") }};
    ($a:literal / $b:literal) => {{
        match Rational::new($a, $b, false) {
            Some(res) => res,
            //Safety: den is non-zero
            None => unreachable!(),
        }
    }};
    (-$a:literal / $b:literal) => {{ -Ratio!($a / $b) }};
}

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    //Numerator
    num: u128,
    //Denominator
    den: NonZeroU128,
    //Negative
    neg: bool,
}

impl Rational {
    fn from_nonzero(num: NonZeroU128, den: NonZeroU128, neg: bool) -> Self {
        let div = gcd(num, den);

            den: NonZeroU128::new(den.get() / div.get()).unwrap(),
            neg,
        }
    }

    pub const fn zero(neg: bool) -> Self {
        // Safety: 1 is non-zero
        Self {
            num: 0,
            den: unsafe { NonZeroU128::new_unchecked(1) },
            neg,
        }
    }

    pub fn new(num: u128, den: u128, neg: bool) -> Option<Self> {
        if num == 0 {
            return Some(Self::zero(neg));
        }

        // Num is non-zero
        let num = NonZeroU128::new(num)?;
        let den = NonZeroU128::new(den)?;
        Some(Self::from_nonzero(num, den, neg))
    }

    pub fn try_new(num: u128, den: u128, neg: bool) -> Result<Self, &'static str> {
        Self::new(num, den, neg).ok_or("Denominator cannot be zero")
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.neg {
            write!(f, "-{}/{}", self.num, self.den.get())
        } else {
            write!(f, "{}/{}", self.num, self.den.get())
        }
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        // Since both den are non-zero, this new is also non.zero
        let den = self.den.get() * rhs.den.get();
        // Xor: Only have sign if only one has
        let neg = self.neg ^ rhs.neg;

        //Safety: den is non-zero
        Self::new(num, den, neg).unwrap()
    }
}

impl std::ops::Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let left_num = self.num * rhs.den.get();
        let right_num = rhs.num * self.den.get();

        let den = self.den.get() * rhs.den.get();

        println!("{} {}", left_num, right_num);

        let (num, neg) = match (self.neg, rhs.neg) {
            (false, false) => (left_num + right_num, false),
            (true, true) => (left_num + right_num, true),
            (true, false) => {
                if left_num > right_num {
                    (left_num - right_num, true)
                } else {
                    (right_num - left_num, false)
                }
            }
            (false, true) => {
                if left_num > right_num {
                    (left_num - right_num, false)
                } else {
                    (right_num - left_num, true)
                }
            }
        };

        //Safety: den is non-zero
        Self::new(num, den, neg).unwrap()
    }
}

impl std::ops::Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut new = self.clone();
        new.neg = true;
        new
    }
}
