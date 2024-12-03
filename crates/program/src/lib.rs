#[cfg(not(feature = "pinocchio"))]
mod vanilla;

#[cfg(not(feature = "pinocchio"))]
pub use vanilla::*;

#[cfg(feature = "pinocchio")]
mod pinocchio;

#[cfg(feature = "pinocchio")]
pub use pinocchio::*;

pub mod bytes;

pub fn try_find_program_address(
    seeds: &[&[u8]],
    program_id: &crate::pubkey::Pubkey,
) -> Option<(crate::pubkey::Pubkey, u8)> {
    let mut bytes: [u8; 32] = [0; 32];
    let mut bump_seed = u8::MAX;
    let result = unsafe {
        crate::syscalls::sol_try_find_program_address(
            seeds as *const _ as *const u8,
            seeds.len() as u64,
            program_id as *const u8,
            &mut bytes as *mut u8,
            &mut bump_seed as *mut u8,
        )
    };

    match result {
        crate::SUCCESS => Some((bytes, bump_seed)),
        _ => None,
    }
}
