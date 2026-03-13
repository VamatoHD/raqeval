use super::Rational;

macro_rules! const_rational {
    ($a:literal / $b:literal) => {{
        use core::num::NonZeroU128;
        Rational {
            num: $a,
            den: match NonZeroU128::new($b) {
                Some(v) => v,
                None => panic!("Invalid rational"),
            },
            neg: false,
        }
    }};
}

pub const E: Rational = const_rational!(848456353 / 312129649);
pub const PI: Rational = const_rational!(1146408 / 364913);
