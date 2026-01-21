pub trait Unsigned {}

macro_rules! impl_unsigned {
    ($($x:ty),+ $(,)?) => {
        $(
            impl Unsigned for $x {}
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);
