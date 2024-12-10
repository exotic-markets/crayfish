#[cfg(not(feature = "pinocchio"))]
mod vanilla;

#[cfg(not(feature = "pinocchio"))]
pub use vanilla::*;

#[cfg(feature = "pinocchio")]
mod pinocchio;

#[cfg(feature = "pinocchio")]
pub use pinocchio::*;

pub mod bytes;
