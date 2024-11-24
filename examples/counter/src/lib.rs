use {
    crayfish_account_macro::account,
    crayfish_accounts::{
        Account, Mut, Program, ReadableAccount, Signer, System, UncheckedAccount, WritableAccount,
    },
    crayfish_context_macro::context,
    crayfish_handler_macro::handlers,
    crayfish_program_id_macro::program_id,
    crayfish_space::Space,
    pinocchio::{entrypoint, msg, program_error::ProgramError},
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    initialize,
    increment
}

#[context]
pub struct InitContext<'a> {
    pub counter: Mut<UncheckedAccount<'a>>,
    pub payer: Signer<'a>,
    pub system: Program<'a, System>,
}

pub fn initialize(ctx: InitContext) -> Result<(), ProgramError> {
    let InitContext {
        counter,
        payer,
        system,
    } = ctx;

    // TODO: Actual account creation
    msg!("{:?}", counter.key());
    msg!("{:?}", payer.key());
    msg!("{:?}", system.key());

    Ok(())
}

#[context]
pub struct IncrementContext<'a> {
    pub counter: Mut<Account<'a, Counter>>,
}

pub fn increment(ctx: IncrementContext) -> Result<(), ProgramError> {
    let IncrementContext { counter } = ctx;

    let mut counter_data = counter.mut_data()?;
    counter_data.count += 1;

    Ok(())
}

#[account]
#[derive(Space)]
pub struct Counter {
    pub count: u64,
}
