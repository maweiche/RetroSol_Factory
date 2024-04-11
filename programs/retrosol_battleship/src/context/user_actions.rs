use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::errors::ErrorCode;
use crate::state::{
    ChestVault, 
    GameRecord
};

#[derive(Accounts)]
#[instruction(selected_square: [u8; 2])]
pub struct PlayerJoinsGame<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerJoinsGame<'info> {
    pub fn join_game(
        &mut self
    ) -> Result<()> {
        
        // if the score_sheet.player_two is not empty, then the game is already full
        if self.chest_vault.score_sheet.player_two != None {
            return Err(ErrorCode::GameIsFull.into());
        }

        // transfer entry fee from signer to chest_vault_account
        let cpi_context: CpiContext<'_, '_, '_, '_, system_program::Transfer<'_>> = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.chest_vault.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, self.chest_vault.entry_fee)?;

        self.chest_vault.score_sheet.player_two = Some(self.signer.key());
        self.chest_vault.score_sheet.current_move = Some(self.chest_vault.score_sheet.player_one);

        Ok(())
    }
}