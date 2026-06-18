#[macro_export]
macro_rules! rat {
    // Zero cases
    (0 $(/ $_:literal)?) => {{ $crate::Rational::zero() }};
    (-0 $(/ $_:literal)?) => {{ $crate::Rational::zero() }};

    //Division by zero
    ($_:literal / 0) => {{ compile_error!("division by zero") }};
    ($_:literal / -0) => {{ compile_error!("division by zero") }};

    //Just the numerator
    (-$a:literal) => {{ rat!($a / 1).const_neg() }};
    ($a:literal) => {{ rat!($a / 1) }};

    //Main case
    (-$a:literal / $b:literal) => { rat!($a / $b).const_neg() };
    ($a:literal / $b:literal) => { const {
        $crate::Rational::unwrap_new(
            ($a as i128).unsigned_abs(),
            ($b as i128).unsigned_abs(),
            false
        )
    }};

    //Invalid case
    ($($tt:tt)*) => {
        compile_error!(concat!("invalid rational literal: ", stringify!($($tt)*)))
    };
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
