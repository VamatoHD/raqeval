use super::Rational;

macro_rules! impl_unsigned {
    ($($x:ty),+ $(,)?) => {
        $(
            // Rational == unsigned
            impl PartialEq<$x> for Rational {
                #[inline(always)]
                fn eq(&self, other: &$x) -> bool {
                    !self.is_neg() && (self.num as u128) == (*other as u128) && self.is_integer()
                }
            }

            // &Rational = unsigned
            impl PartialEq<$x> for &Rational {
                #[inline(always)]
                fn eq(&self, other: &$x) -> bool {
                    // Dereferences &&Rational in order to call other method
                    (*self).eq(other)
                }
            }

            // unsigned = Rational
            impl PartialEq<Rational> for $x {
                #[inline(always)]
                fn eq(&self, other: &Rational) -> bool {
                    other == self
                }
            }

            // unsigned = &Rational
            impl PartialEq<&Rational> for $x {
                #[inline(always)]
                fn eq(&self, other: &&Rational) -> bool {
                    other == self
                }
            }

            impl From<$x> for Rational {
                #[inline(always)]
                fn from(value: $x) -> Self {
                    //Silently truncates usize in case it is 256 bits - Fix later
                    Rational::unwrap_new(value as _, 1)
                }
            }
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);
