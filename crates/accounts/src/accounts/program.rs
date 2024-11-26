use std::marker::PhantomData;

use crayfish_errors::Error;
use crayfish_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref};

use crate::{FromAccountInfo, ProgramId, ReadableAccount};

///
/// Checks:
/// * `account_info.key == expected_program`
/// * `account_info.executable == true`
pub struct Program<'a, T>
where
    T: ProgramId,
{
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where
    T: ProgramId,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.key() != &T::ID {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        if !info.executable() {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> AsRef<RawAccountInfo> for Program<'a, T>
where
    T: ProgramId,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<'a, T> ReadableAccount for Program<'a, T>
where
    T: ProgramId,
{
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
