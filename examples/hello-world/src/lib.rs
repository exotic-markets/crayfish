use {
    crayfish_handler_macro::handlers,
    crayfish_program_id_macro::program_id,
    pinocchio::{entrypoint, msg, program_error::ProgramError},
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    hello_world,
}

pub fn hello_world() -> Result<(), ProgramError> {
    msg!("Hello World");

    Ok(())
}
