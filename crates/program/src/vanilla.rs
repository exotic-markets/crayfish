pub use {
    nostd_system_program as system_program,
    solana_nostd_entrypoint::{
        basic_panic_impl, entrypoint_nostd, noalloc_allocator,
        solana_program::{entrypoint::ProgramResult, *},
        Ref, RefMut,
    },
};

pub mod sysvars {
    pub use solana_nostd_entrypoint::solana_program::sysvar::*;
}

pub type RawAccountInfo = solana_nostd_entrypoint::NoStdAccountInfo;
pub type Account = solana_nostd_entrypoint::AccountInfoC;
pub type Instruction = solana_nostd_entrypoint::InstructionC;
pub type Signer<'a, 'b> = &'a [&'b [u8]];
pub type AccountMeta = solana_nostd_entrypoint::AccountMetaC;

#[macro_export]
macro_rules! program_entrypoint {
    ($name: ident) => {
        use solana_nostd_entrypoint::NoStdAccountInfo;

        $crate::entrypoint_nostd!(process_instruction, 32);
        $crate::noalloc_allocator!();
        $crate::basic_panic_impl!();
    };
}

impl crate::ToMeta for RawAccountInfo {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta {
        let mut meta = if is_signer {
            self.to_meta_c_signer()
        } else {
            self.to_meta_c()
        };

        meta.is_writable = is_writable;
        meta
    }
}
