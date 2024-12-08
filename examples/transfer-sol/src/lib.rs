use {
    bytemuck::{Pod, Zeroable},
    crayfish_accounts::{Mut, Program, Signer, System, SystemAccount},
    crayfish_context::args::Args,
    crayfish_context_macro::context,
    crayfish_handler_macro::handlers,
    crayfish_program::program_error::ProgramError,
    crayfish_program_id_macro::program_id,
    crayfish_traits::{Lamports, SystemCpi},
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    transfer_sol_with_cpi,
    transfer_sol_with_program
}

#[context]
pub struct TransferContext {
    pub payer: Mut<Signer>,
    pub recipient: Mut<SystemAccount>,
}

#[context]
pub struct SystemContext {
    pub system: Program<System>,
}

pub fn transfer_sol_with_cpi(
    amount: Args<u64>,
    ctx: TransferContext,
    _: SystemContext,
) -> Result<(), ProgramError> {
    ctx.payer.transfer(&ctx.recipient, *amount)?;

    Ok(())
}

pub fn transfer_sol_with_program(
    amount: Args<u64>,
    ctx: TransferContext,
) -> Result<(), ProgramError> {
    ctx.payer.send(&ctx.recipient, *amount)?;

    Ok(())
}
