use {crate::ProgramId, crayfish_program::system_program};

pub struct System;

impl ProgramId for System {
    const ID: crayfish_program::pubkey::Pubkey = system_program::ID;
}
