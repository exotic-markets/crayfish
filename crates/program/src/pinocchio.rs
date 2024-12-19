pub use {
    pinocchio::{
        account_info::{Ref, RefMut},
        instruction::AccountMeta,
        *,
    },
    pinocchio_system as system_program,
};

pub type RawAccountInfo = account_info::AccountInfo;
pub type SignerSeeds<'a, 'b> = instruction::Signer<'a, 'b>;

pub use pinocchio_pubkey::declare_id;

#[macro_export]
macro_rules! program_entrypoint {
    ($name: ident) => {
        use program::entrypoint;

        $crate::entrypoint!(process_instruction);
    };
}

impl crate::ToMeta for RawAccountInfo {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta {
        AccountMeta::new(self.key(), is_writable, is_signer)
    }
}
