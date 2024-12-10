use {
    bytemuck::{Pod, Zeroable},
    crayfish_account_macro::account,
    crayfish_accounts::{
        Account, FromAccountInfo, Mut, Program, ReadableAccount, Signer, System, WritableAccount,
    },
    crayfish_context::args::Args,
    crayfish_context_macro::{args, context},
    crayfish_handler_macro::handlers,
    crayfish_program::{msg, program_error::ProgramError},
    crayfish_program_id_macro::program_id,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct InitArgs {
    pub value: u64,
}

#[context]
#[args(InitArgs)]
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
#[args(value: u64, other_value: u64)]
pub struct SetValueContext {
    pub buffer: Mut<Account<Buffer>>,
}

handlers! {
    initialize,
    set_value,
    set_and_add_values,
}

pub fn initialize(ctx: InitContext) -> Result<(), ProgramError> {
    msg!("{}", ctx.args.value);

    Ok(())
}

pub fn set_value(ctx: SetValueContext, more_args: Args<u64>) -> Result<(), ProgramError> {
    ctx.buffer.mut_data()?.value = ctx.args.value;
    msg!("{}", *more_args);

    Ok(())
}

pub fn set_and_add_values(
    ctx_a: SetValueContext,
    ctx_b: SetValueContext,
) -> Result<(), ProgramError> {
    ctx_a.buffer.mut_data()?.value = ctx_a.args.value;
    ctx_b.buffer.mut_data()?.value = ctx_b.args.value;

    msg!(
        "{}",
        ctx_a.buffer.data()?.value + ctx_b.buffer.data()?.value
    );

    Ok(())
}

#[account]
pub struct Buffer {
    pub value: u64,
}

impl Buffer {
    const SPACE: usize = std::mem::size_of::<Buffer>();
}
