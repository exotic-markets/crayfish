use {
    bytemuck::{Pod, Zeroable},
    typhoon_accounts::{Mut, Program, Signer, System, SystemAccount},
    typhoon_context::args::Args,
    typhoon_context_macro::context,
    typhoon_handler_macro::handlers,
    typhoon_program::program_error::ProgramError,
    typhoon_program_id_macro::program_id,
    typhoon_traits::{Lamports, SystemCpi},
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
