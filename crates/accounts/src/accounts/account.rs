use {
    crate::{readable::Readable, FromAccountInfo, Owner, ReadableAccount},
    bytemuck::Pod,
    crayfish_errors::Error,
    crayfish_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref},
    std::marker::PhantomData,
};

pub struct Account<'a, T>
where
    T: Owner + Pod,
{
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Pod,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &T::OWNER {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<T> AsRef<RawAccountInfo> for Account<'_, T>
where
    T: Owner + Pod,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: Owner + Pod,
{
    type DataType = T;

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
        let data = self.info.try_borrow_data()?;

        Ref::filter_map(data, |data| T::read(data)).map_err(|_| ProgramError::InvalidAccountData)
    }
}
