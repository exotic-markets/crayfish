use {
    crate::HandlerContext,
    crayfish_program::{program_error::ProgramError, RawAccountInfo},
};

impl<'a> HandlerContext<'a> for &'a [RawAccountInfo] {
    fn from_entrypoint(
        accounts: &mut &'a [RawAccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        Ok(accounts)
    }
}
