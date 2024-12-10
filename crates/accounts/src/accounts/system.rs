use {
    crate::{FromAccountInfo, ReadableAccount},
    crayfish_errors::Error,
    crayfish_program::{
        program_error::ProgramError, pubkey::Pubkey, system_program, RawAccountInfo, Ref,
    },
};

pub struct SystemAccount<'a> {
    info: &'a RawAccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &system_program::ID {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(SystemAccount { info })
    }
}

impl AsRef<RawAccountInfo> for SystemAccount<'_> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl ReadableAccount for SystemAccount<'_> {
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn owner(&self) -> &Pubkey {
        self.info.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}
