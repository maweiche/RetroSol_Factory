use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::state::ChestVault;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut, close = signer)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>CloseAccount<'info> {
    pub fn close_game(
        &mut self, 
    ) -> Result<()> {
        let winner = self.chest_vault.score_sheet.winner;

        if winner == None &&  self.chest_vault.score_sheet.player_two != None{
            return Err(ErrorCode::GameIsNotOver.into());
        }

        // if the signer is not the creator of the chest, panic
        if self.signer.key() != self.chest_vault.authority {
            return Err(ErrorCode::NotCreator.into());
        }

        // Close the game_data_account and transfer all lamports to the signer
        self.chest_vault.close(self.signer.to_account_info())?;

        Ok(())
    }
}