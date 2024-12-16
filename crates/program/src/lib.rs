#[cfg(not(feature = "pinocchio"))]
mod vanilla;

#[cfg(not(feature = "pinocchio"))]
pub use vanilla::*;

#[cfg(feature = "pinocchio")]
mod pinocchio;

#[cfg(feature = "pinocchio")]
pub use pinocchio::*;

pub mod bytes;

pub trait ToMeta {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta;
}

pub const UNINIT_BYTE: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [std::mem::MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}
