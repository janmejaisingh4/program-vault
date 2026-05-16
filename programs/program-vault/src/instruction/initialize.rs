use crate::state::VaultState;
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(  // init the vault state account
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>, // vault state account to store the vault information

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )] // init the vault account
    pub vault: SystemAccount<'info>, // vault account is a system account, we will use it to store the funds

    pub system_program: Program<'info, System>,// we need the system program to create the vault account
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &IntializeBumps) -> Result<()> {
        //save data to the state account
        self.vault_state.initialize.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}