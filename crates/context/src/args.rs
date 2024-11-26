use {
    crate::HandlerContext,
    bytemuck::Pod,
    crayfish_program::{program_error::ProgramError, RawAccountInfo},
    std::{mem::size_of, ops::Deref},
};

#[repr(C, align(8))]
pub struct Args<'a, T>(&'a T); //Constraint trait Aligned

impl<'a, T> Args<'a, T> {
    pub fn new(arg: &'a T) -> Self {
        Args(arg)
    }
}

impl<'a, T> Deref for Args<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> HandlerContext<'a> for Args<'a, T>
where
    T: Pod,
{
    fn from_entrypoint(
        _accounts: &mut &'a [RawAccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        let arg: &T = bytemuck::try_from_bytes(instruction_data)
            .map_err(|_err| ProgramError::AccountBorrowFailed)?; //TODO

        let (_, remaining) = instruction_data.split_at(size_of::<T>());
        *instruction_data = remaining;

        Ok(Args(arg))
    }
}
