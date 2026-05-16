use anchor_lang::prelude::*;

pub mod constants;
pub mod instruction;
pub mod state;

pub use constants::*;
pub use instruction::*;
pub use state::*;

declare_id!("298AoxcTPxtnyx7a5k12wPEpvVHmhSKoAbEv81jwTWc3");

#[program]
pub struct program_vault {
    use crate::instruction::{Deposit, Initialize, Withdraw, CloseAccount};

use super::*;

    //initialize the vault
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps);
        Ok(())
    }

    //deposit into the vault

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)
    }

    //withdraw from the vault

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(amount)
    }


    //close the vault
    pub fn closeAccount(ctx: Context<CloseAccount>) -> Result<()>{
        ctx.accounts.close_account()
    }

}
