use account_info::AccountInfo;
pub use {
    account_info::{Ref, RefMut},
    pinocchio::*,
    pinocchio_system as system_program,
};

pub type RawAccountInfo = AccountInfo;
pub type Signer<'a, 'b> = instruction::Signer<'a, 'b>;

pub use {pinocchio::pubkey::Pubkey, pinocchio_pubkey::declare_id};
