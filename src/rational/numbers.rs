use super::Rational;

macro_rules! impl_unsigned {
    ($($x:ty),+ $(,)?) => {
        $(
            // Rational == unsigned
            impl PartialEq<$x> for Rational {
                #[inline]
                fn eq(&self, other: &$x) -> bool {
                    self.num == (*other as u128) && !self.is_neg() && self.is_integer()
                }
            }

            // &Rational = unsigned
            impl PartialEq<$x> for &Rational {
                #[inline]
                fn eq(&self, other: &$x) -> bool {
                    // Dereferences &&Rational in order to call other method
                    (*self).eq(other)
                }
            }

            // unsigned = Rational
            impl PartialEq<Rational> for $x {
                #[inline]
                fn eq(&self, other: &Rational) -> bool {
                    other == self
                }
            }

            // unsigned = &Rational
            impl PartialEq<&Rational> for $x {
                #[inline]
                fn eq(&self, other: &&Rational) -> bool {
                    other == self
                }
            }
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);
