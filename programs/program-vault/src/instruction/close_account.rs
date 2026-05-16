use anchor_lang::{prelude::*, system_program::{System, Transfer, transfer}};
use crate::state::VaultState;

#[derive(Accounts)]
pub struct CloseAccount<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    //vault PDA account
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]

    pub vault: SystemAccount<'info>,

    //vault state account
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.state_bump
    )]

    //we need to access the vault state account to get the vault bump and also to update the total amount in the vault after withdraw
    pub vault_state: Account<'info, VaultState>,

    system_program: Program<'info, System>,
}

impl <'info>CloseAccount<'info> {
    pub fn close_account(&mut self) -> Result<()>{
        let cpi_account: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(), 
            to: self.user.to_account_info() 
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        //we need to transfer all the lamports from the vault to the user before closing the account
        let signer_seeds = &[&seeds[..]];

        //transfer all the lamports from the vault to the user before closing the account
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_account,
            signer_seeds,
        );
        transfer(cpi_ctx, self.vault.lamports())?;//transfer all the lamports from the vault to the user before closing the account
        Ok(())
    }
}