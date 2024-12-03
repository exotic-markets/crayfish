use {
    crate::{FromAccountInfo, Owner},
    anchor_lang::{prelude::ProgramError, AccountDeserialize, Discriminator},
    crayfish_errors::Error,
    crayfish_program::RawAccountInfo,
};

//TODO zero copy
pub struct AnchorAccount<'a, T>
where
    T: AccountDeserialize,
{
    info: &'a RawAccountInfo,
    data: T,
}

impl<'a, T> AnchorAccount<'a, T>
where
    T: AccountDeserialize,
{
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<'a, T> FromAccountInfo<'a> for AnchorAccount<'a, T>
where
    T: Owner + AccountDeserialize + Discriminator,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &T::OWNER {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        let data = T::try_deserialize(&mut info.try_borrow_data()?.as_ref())?;

        Ok(AnchorAccount { info, data })
    }
}

impl<'a, T> AsRef<RawAccountInfo> for AnchorAccount<'a, T>
where
    T: AccountDeserialize,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}
