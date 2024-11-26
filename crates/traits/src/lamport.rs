use {
    crayfish_accounts::{Mut, Signer, SignerAccount, WritableAccount},
    crayfish_program::program_error::ProgramError,
};

pub trait Lamports: WritableAccount + SignerAccount {
    fn send(&self, to: &impl WritableAccount, amount: u64) -> Result<(), ProgramError> {
        self.mut_lamports()?
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        to.mut_lamports()?
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }

    fn send_all(&self, to: &impl WritableAccount) -> Result<(), ProgramError> {
        let lamports = *self.lamports()?;

        self.mut_lamports()?
            .checked_sub(lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        to.mut_lamports()?
            .checked_add(lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}

impl<'a> Lamports for Mut<Signer<'a>> {}
