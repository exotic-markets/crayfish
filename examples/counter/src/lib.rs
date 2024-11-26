use crayfish_account_macro::account;
use crayfish_accounts::{Account, FromAccountInfo, Mut, Program, Signer, System, WritableAccount};
use crayfish_context_macro::context;
use crayfish_handler_macro::handlers;
use crayfish_program_id_macro::program_id;
use crayfish_space::{InitSpace, Space};
use pinocchio::{entrypoint, msg, program_error::ProgramError};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext<'a> {
    pub payer: Signer<'a>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::INIT_SPACE
    )]
    pub counter: Mut<Account<'a, Counter>>,
    pub system: Program<'a, System>,
}

#[context]
pub struct IncrementContext<'a> {
    pub counter: Mut<Account<'a, Counter>>,
}

handlers! {
    initialize,
    increment
}

pub fn initialize(_: InitContext) -> Result<(), ProgramError> {
    Ok(())
}

pub fn increment(IncrementContext { counter }: IncrementContext) -> Result<(), ProgramError> {
    counter.mut_data()?.count += 1;

    Ok(())
}

#[account]
#[derive(Space)]
pub struct Counter {
    pub count: u64,
}
