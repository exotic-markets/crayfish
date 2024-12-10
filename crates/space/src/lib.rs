use typhoon_program::pubkey::Pubkey;
pub use typhoon_space_macro::Space;

/// Defines the space of an account for initialization.
pub trait InitSpace {
    const INIT_SPACE: usize;
}

macro_rules! impl_min_space {
    ($ty:ident => $space:literal) => {
        impl InitSpace for $ty {
            const INIT_SPACE: usize = $space;
        }
    };
    (($($ty:ident),+) => $space:literal) => {
        $(
            impl_min_space!($ty => $space);
        )+

    };
}

impl_min_space!((i8, u8, bool) => 1);
impl_min_space!((i16, u16) => 2);
impl_min_space!((i32, u32, f32) => 4);
impl_min_space!((i64, u64, f64) => 8);
impl_min_space!((i128, u128) => 16);
impl_min_space!(Pubkey => 32);

impl<T: InitSpace> InitSpace for Option<T> {
    const INIT_SPACE: usize = 1 + T::INIT_SPACE;
}

#[doc(hidden)]
pub const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}
