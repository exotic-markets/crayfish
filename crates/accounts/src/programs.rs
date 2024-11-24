use crayfish_program::system_program;

use crate::ProgramId;

pub struct System;

impl ProgramId for System {
    const ID: crayfish_program::Pubkey = system_program::ID;
}

pub struct BpfLoaderUpgradeable;

impl ProgramId for BpfLoaderUpgradeable {
    // TODO: PR pinocchio to add native programs
    const ID: crayfish_program::Pubkey = [
        5, 135, 132, 191, 20, 139, 164, 40, 47, 176, 18, 87, 72, 136, 169, 241, 83, 160, 125, 173,
        247, 101, 192, 69, 92, 154, 151, 3, 128, 0, 0, 0,
    ];
}
