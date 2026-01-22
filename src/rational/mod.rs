mod gcd;
use gcd::gdc_nonzerou128 as gcd;

mod unsigned;
use unsigned::Unsigned;

use core::num::NonZeroU128;

macro_rules! Ratio {
    //Handling zero cases
    (0) => {{ Rational::zero() }};
    (-0) => {{ Rational::zero() }};
    (0 / $b:expr) => {{ Rational::zero() }};
    (- 0 / $b:expr) => {{ Rational::zero() }};
    ($a:literal / 0) => {{ compile_error!("Division by zero.") }};
    //Handling just the numerator cases
    (-$a:literal) => {{ -Ratio!($a) }};
    ($a:literal) => {{ Ratio!($a / 1) }};

    (-$a:literal / $b:literal) => {{ -Ratio!($a / $b) }};
    ($a:literal / $b:literal) => {{
        match Rational::new($a as u128, $b as u128, false) {
            Some(res) => res,
            //Safety: den is non-zero
            None => unreachable!(),
        }
    }};

    () => {{ compile_error!("Invalid Ratio") }};
}

macro_rules! to_nonzeroU128 {
    (0) => {
        compile_error!("Unable to convert into non-zero")
    };
    ($x:expr) => {{ ::core::num::NonZeroU128::new($x).expect("Is non-zero") }};
}

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    //Numerator
    num: u128,
    //Denominator
    den: NonZeroU128,
    //Negative, must be false if num == 0
    neg: bool,
}

impl Rational {
    fn from_nonzero(num: NonZeroU128, den: NonZeroU128, neg: bool) -> Self {
        let mut new = Self {
            num: num.get(),
            den,
            neg,
        };
        new.reduce_in_place();
        new
    }

    pub const fn zero() -> Self {
        // Safety: 1 is non-zero
        Self {
            num: 0,
            den: to_nonzeroU128!(1),
            neg: false,
        }
    }

    pub fn new(num: u128, den: u128, neg: bool) -> Option<Self> {
        if num == 0 {
            return Some(Self::zero());
        }

        // Num is non-zero
        let num = NonZeroU128::new(num)?;
        let den = NonZeroU128::new(den)?;
        Some(Self::from_nonzero(num, den, neg))
    }

    pub fn try_new(num: u128, den: u128, neg: bool) -> Result<Self, &'static str> {
        Self::new(num, den, neg).ok_or("Denominator cannot be zero")
    }

    pub fn reduce_in_place(&mut self) -> &mut Self {
        if self.num == 0 {
            //SAFETY: 1 is non-zero
            self.den = to_nonzeroU128!(1);
            self.neg = false;
            return self;
        }
        //SAFETY: self.num is non-zero (checked) and so is self.den (non-zero type)
        let div = gcd(to_nonzeroU128!(self.num), self.den);

        self.num /= div;
        //SAFETY: self.den and div are non-zero
        self.den = to_nonzeroU128!(self.den.get() / div);
        self
    }

    pub fn reduce(&self) -> Self {
        let mut new = self.clone();
        new.reduce_in_place();
        new
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
        // Since both den are non-zero, this new is also non-zero
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
        let left_num = dbg!(self.num * rhs.den.get());
        let right_num = dbg!(rhs.num * self.den.get());

        let den = self.den.get() * rhs.den.get();

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
        new.neg = !new.neg && new.num != 0;
        new
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        (self.num == 0 && other.num == 0)
            || (self.neg == other.neg && {
                let a = self.reduce();
                let b = other.reduce();
                a.num == b.num && a.den == b.den
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Rational;
    const MAX: u128 = !0;

    #[test]
    fn eq_test() {
        assert_eq!(Rational::zero(), Rational::zero());
        assert_eq!(Ratio!(2 / 3), Ratio!(4 / 6));
        assert_eq!(Ratio!(0), Ratio!(0));
        assert_ne!(Ratio!(-2 / 3), Ratio!(2 / 3));
        assert_ne!(Ratio!(-4 / 3), Ratio!(8 / 6));
        assert_ne!(Ratio!(-0 / 100), Ratio!(8 / 2));
        assert_ne!(Ratio!(0 / MAX), Ratio!(8 / 2));
        assert_eq!(Ratio!(0 / MAX), Ratio!(-0));
    }

    #[test]
    fn neg_test() {
        assert_eq!(Ratio!(0), -Ratio!(0));
        assert_eq!(Ratio!(-1), -Ratio!(1));
        assert_eq!(
            Rational::from_nonzero(to_nonzeroU128!(1), to_nonzeroU128!(1), true),
            -Rational::from_nonzero(to_nonzeroU128!(1), to_nonzeroU128!(1), false)
        );
        assert_ne!(
            Rational::from_nonzero(to_nonzeroU128!(1), to_nonzeroU128!(1), true),
            -Rational::from_nonzero(to_nonzeroU128!(1), to_nonzeroU128!(1), true)
        );
        assert_eq!({ -Rational::zero() }.neg, false);
    }

    #[test]
    fn add_test() {
        assert_eq!(Ratio!(1) + Ratio!(1), Ratio!(2));
        assert_eq!(Ratio!(1 / 2) + Ratio!(1 / 2), Ratio!(1));
        assert_eq!(Ratio!(-1 / 2) + Ratio!(1 / 2), Ratio!(0));
        assert_eq!(Ratio!(99 / 100) + Ratio!(1 / 100), Ratio!(1));
        assert_eq!(Ratio!(3 / 5) + Ratio!(7 / 11), Ratio!(68 / 55));
        assert_eq!(Ratio!(3 / 5) + Ratio!(-7 / 11), Ratio!(-2 / 55));
        assert_eq!(Ratio!(-7 / 11) + Ratio!(3 / 5), Ratio!(-2 / 55));
        assert_eq!(Ratio!(-2) + Ratio!(-2), Ratio!(-4));
    }

    #[test]
    fn reduce_test() {
        //Rational::new automatically reduces
        assert_eq!(Ratio!(0).reduce(), Ratio!(0));
        assert_eq!(Ratio!(2 / 4), Ratio!(1 / 2));
        assert_eq!(Ratio!(2 / 4), Ratio!(1 / 2));
        assert_eq!(Ratio!(100 / 50), Ratio!(8 / 4));
        assert_eq!(Ratio!(-100 / 50), -Ratio!(8 / 4));
    }

    #[test]
    fn mul_test() {
        assert_eq!(Ratio!(0) * Ratio!(0), Ratio!(0));
        assert_eq!(Ratio!(2) * Ratio!(4), Ratio!(8));
        assert_eq!(Ratio!(-2) * Ratio!(1 / 2), Ratio!(-1));
        assert_eq!(Ratio!(-1) * Ratio!(-1), Ratio!(1));
    }
}
