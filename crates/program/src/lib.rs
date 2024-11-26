#[cfg(not(feature = "pinocchio"))]
mod vanilla;

#[cfg(not(feature = "pinocchio"))]
pub use vanilla::*;

#[cfg(feature = "pinocchio")]
mod pinocchio;

#[cfg(feature = "pinocchio")]
pub use pinocchio::*;

// pub trait Aligned {
//     const fn check_alignement() {
//         assert_eq!(std::mem::align_of::<Self>(), 8)
//     }
// }

pub trait Aligned {
    const ALIGNED: ();
}

// impl<T> Aligned for T {
//     const ALIGNED = assert_eq!(std::mem::align_of::<T>(), 8);
// }

// const fn assert_alignment<T>() {
//     if std::mem::align_of::<T>() != 8 {
//         panic!("qsdsqd")
//     }
// }

// /// Re-interprets `&[u8]` as `&T`.
// ///
// /// ## Failure
// ///
// /// * If the slice's length isn’t exactly the size of the new type
// #[inline]
// pub(crate) fn try_from_bytes<T: Copy>(s: &[u8]) -> Result<&T, ProgramError> {
//     if s.len() != size_of::<T>() {
//         Err(ProgramError::InvalidAccountData) //TODO maybe use a better error here
//     } else {
//         Ok(unsafe { &*(s.as_ptr() as *const T) })
//     }
// }
