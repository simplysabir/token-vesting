use anchor_lang::prelude::*;

impl EscrowAccount {
    // Define the maximum size of the EscrowAccount struct in bytes.
    pub const MAX_SIZE : usize = 32 + 32 + 8 + 32 + 8 + 32 + 1;
}

#[account]
#[derive(Default)]
pub struct EscrowAccount {
    pub owner : Pubkey,
    pub id : Pubkey,
    pub amount : u64,
    pub token : Pubkey,
    pub time : u64,
    pub withdrawer : Pubkey,
    pub bump : u8,
}

// The EscrowAccount struct represents the account state for an escrow.