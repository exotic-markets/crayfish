use crayfish_accounts::{Mut, ReadableAccount, SystemAccount, WritableAccount};
use crayfish_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program::instructions::{Allocate, Assign, CreateAccount, Transfer},
    sysvars::{rent::Rent, Sysvar},
    Signer,
};

pub trait SystemCpi: WritableAccount {
    fn allocate(&self, new_space: u64) -> Result<(), ProgramError> {
        Allocate {
            account: self.as_ref(),
            space: new_space,
        }
        .invoke()
    }

    fn assign(&self, owner: &Pubkey) -> Result<(), ProgramError> {
        Assign {
            account: self.as_ref(),
            owner,
        }
        .invoke()
    }

    fn create_account(
        &self,
        payer: &impl ReadableAccount,
        owner: &Pubkey,
        space: u64,
        seeds: Option<&[Signer]>,
    ) -> Result<(), ProgramError> {
        CreateAccount {
            from: payer.as_ref(),
            lamports: Rent::get()?.minimum_balance(space as usize),
            owner,
            space,
            to: self.as_ref(),
        }
        .invoke_signed(seeds.unwrap_or_default())
    }

    fn transfer(&self, to: &impl WritableAccount, amount: u64) -> Result<(), ProgramError> {
        Transfer {
            from: self.as_ref(),
            lamports: amount,
            to: to.as_ref(),
        }
        .invoke()
    }
}

impl<'a> SystemCpi for Mut<SystemAccount<'a>> {}
