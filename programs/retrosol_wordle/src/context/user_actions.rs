// Functions
// 1 - user creates pda / starts game
// 2 - user_guesses letter
// 3 - user withdraws winnings

use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::{
    state::{
        ChestVault,
        GameRecord,
    },
    errors::GameError,
};

#[derive(Accounts)]
pub struct PlayerStartsGame<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerStartsGame<'info> {
    pub fn start_game(
        &mut self,
    ) -> Result<()> {

        /*
        
            -Player starts the game
            -Player transfers the entry fee to the ChestVault

            Checks:
            -Game has not been solved
            -Player has not already started the game
        */

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.chest_vault.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, self.chest_vault.entry_fee)?;
        
        Ok(())
    }
}