use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::ChestVault;

#[derive(Accounts)]
#[instruction(
    chest_reward: u64, 
    password: String, 
    max_attempts: u8, 
    entry_fee: u64
)]
pub struct HangmanInit<'info> {
    #[account(
        init,
        seeds = [
            b"chestVault",
            signer.key().as_ref()
        ],
        bump,
        payer = signer,
        space = ChestVault::INIT_SPACE
    )]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>HangmanInit<'info> {
    // Initializes the tipboard
    pub fn initialize_hangman(
        &mut self,
        chest_reward: u64, 
        password: String, 
        max_attempts: u8, 
        entry_fee: u64
    ) -> Result<()> {

        /*
        
            -Create a new Hangman Game and set the params of the ChestVault:
            -Invoke the transfer of the chest_reward to the ChestVault from the creator

        */

        self.chest_vault.set_inner(
            ChestVault {
                creator: self.signer.key(),
                chest_reward,
                password,
                max_attempts_left: max_attempts,
                entry_fee,
                players: vec![],
            }
        );

        let cpi_context: CpiContext<'_, '_, '_, '_, system_program::Transfer<'_>> = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.chest_vault.to_account_info().clone(),
            },
        );
        
        system_program::transfer(cpi_context, chest_reward)?;

        Ok(())
    }
}