use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info>{

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

    //we need to access the vault state account to get the vault bump and also to update the total amount in the vault after deposit
    pub vault_state: Account<'info, VaultState>,

    system_program: Program<'info, System>,
}

//we will implement the deposit function in the context of the Deposit struct
impl <'info>Deposit<'info> {
    pub fn deposit(&mut self, amount: u64)-> Result<()>{
        let cpi_account: Transfer<'_> = Transfer {
            from: self.user.to_account_info(), 
            to: self.vault.to_account_info() 
        };

        let cpi_ctx: CpiContext<'_, '_, '_, 'info, Transfer<'_>> = CpiContext::new(self.system_program.to_account_info(), cpi_account);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}