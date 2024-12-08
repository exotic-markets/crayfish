use {
    bytemuck::{Pod, Zeroable},
    crayfish_account_macro::account,
    crayfish_accounts::{Account, FromAccountInfo, Mut, Program, Signer, System, WritableAccount},
    crayfish_context_macro::{context, instruction},
    crayfish_handler_macro::handlers,
    crayfish_program::program_error::ProgramError,
    crayfish_program_id_macro::program_id,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext {
    pub payer: Signer,
    #[constraint(
        init,
        payer = payer,
        space = Buffer::SPACE
    )]
    pub buffer: Mut<Account<Buffer>>,
    pub system: Program<System>,
}

#[context]
#[instruction(value: u64, other_value: u64,)]
pub struct SetValueContext {
    pub buffer: Mut<Account<Buffer>>,
}

handlers! {
    initialize,
    set_value
}

pub fn initialize(_: InitContext) -> Result<(), ProgramError> {
    Ok(())
}

pub fn set_value(ctx: SetValueContext) -> Result<(), ProgramError> {
    ctx.buffer.mut_data()?.value = ctx.args.value;

    Ok(())
}

#[account]
pub struct Buffer {
    pub value: u64,
}

impl Buffer {
    const SPACE: usize = std::mem::size_of::<Buffer>();
}
