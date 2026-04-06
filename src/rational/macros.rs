macro_rules! rat {
    // Zero cases
    (0 $(/ $_:literal)?) => {{ $crate::Rational::zero() }};
    (-0 $(/ $_:literal)?) => {{ $crate::Rational::zero() }};

    //Division by zero
    ($_:literal / 0) => {{ compile_error!("division by zero") }};
    ($_:literal / -0) => {{ compile_error!("division by zero") }};

    //Just the numerator
    (-$a:literal) => {{ -rat!($a / 1) }};
    ($a:literal) => {{ rat!($a / 1) }};

    //Main case
    (-$a:literal / $b:literal) => {{ -rat!($a / $b) }};
    ($a:literal / $b:literal) => {{
        //Todo: if NUM or DEN are bigger than i128
        const NUM: i128 = $a;
        const DEN: i128 = $b;
        const SIGN: bool = NUM.is_negative() ^ DEN.is_negative();
        const ABS_NUM: u128 = NUM.unsigned_abs();
        const ABS_DEN: u128 = DEN.unsigned_abs();

        match $crate::Rational::new(ABS_NUM, ABS_DEN, SIGN) {
            Ok(res) => res,
            //Safety: den is non-zero
            Err(_) => unreachable!(),
        }
    }};

    //Invalid case
    ($($tt:tt)*) => {
        compile_error!(concat!("invalid rational literal: ", stringify!($($tt)*)))
    };
}

macro_rules! to_nonzeroU128 {
    (0) => {
        compile_error!("Unable to convert into non-zero")
    };
    ($x:expr) => {{ ::core::num::NonZeroU128::new($x).expect("Is non-zero") }};
}

macro_rules! impl_ops {
    ($op_trait:ident, $op_fn:ident) => {
        // &Rational + &Rational
        impl std::ops::$op_trait<&Rational> for &Rational {
            type Output = Rational;
            #[inline]
            fn $op_fn(self, rhs: &Rational) -> Self::Output {
                self.clone().$op_fn(rhs.clone())
            }
        }

        // Rational + &Rational
        impl std::ops::$op_trait<&Rational> for Rational {
            type Output = Rational;
            #[inline]
            fn $op_fn(self, rhs: &Rational) -> Self::Output {
                self.$op_fn(rhs.clone())
            }
        }

        // &Rational + Rational
        impl std::ops::$op_trait<Rational> for &Rational {
            type Output = Rational;
            #[inline]
            fn $op_fn(self, rhs: Rational) -> Self::Output {
                self.clone().$op_fn(rhs)
            }
        }
    };
}

pub(super) use impl_ops;
pub(crate) use rat;
pub(super) use to_nonzeroU128;
