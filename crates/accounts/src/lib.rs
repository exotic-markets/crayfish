mod accounts;
mod programs;
mod readable;

pub use accounts::*;
pub use programs::*;
pub use readable::*;

use crayfish_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref, RefMut};
use sealed::Sealed;

pub trait ProgramId {
    const ID: Pubkey;
}

pub trait Owner {
    const OWNER: Pubkey;
}

pub trait ReadableAccount: AsRef<RawAccountInfo> {
    type DataType: ?Sized + Readable;

    fn key(&self) -> &Pubkey;
    fn owner(&self) -> &Pubkey;
    fn lamports(&self) -> Result<Ref<u64>, ProgramError>;
    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError>;
}

pub trait WritableAccount: ReadableAccount + Sealed {
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError>;
    fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError>;
    fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError>;
}

pub trait SignerAccount: ReadableAccount + Sealed {}

mod sealed {
    use crayfish_program::RawAccountInfo;

    use super::{Mut, ReadableAccount, Signer};

    pub trait Sealed {}

    impl<T> Sealed for Mut<T> where T: ReadableAccount + AsRef<RawAccountInfo> {}
    impl<'a> Sealed for Signer<'a> {}
}

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError>;
}
