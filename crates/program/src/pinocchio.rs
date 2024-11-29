use account_info::AccountInfo;
pub use {
    account_info::{Ref, RefMut},
    pinocchio::*,
    pinocchio_system as system_program,
};

pub type RawAccountInfo = AccountInfo;
pub type Signer<'a, 'b> = instruction::Signer<'a, 'b>;

pub use pinocchio_pubkey::declare_id;

#[macro_export]
macro_rules! program_entrypoint {
    ($name: ident) => {
        use crayfish_program::entrypoint;

        $crate::entrypoint!(process_instruction);
    };
}
