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
pub struct PlayerJoinsGame<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerJoinsGame<'info> {
    pub fn join_game(
        &mut self,
    ) -> Result<()> {

        /*
        
        */

        // if the score_sheet.player_two is not empty, then the game is already full
        if self.chest_vault.score_sheet.player_two != Pubkey::default() {
            panic!("Game is already full");
        }

        // Clone the chest_vault_account before passing it as an argument
        let chest_vault_account_info = self.chest_vault.to_account_info().clone();

        // transfer entry fee from signer to chest_vault_account
        let cpi_context: CpiContext<'_, '_, '_, '_, system_program::Transfer<'_>> = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: chest_vault_account_info,
            },
        );

        system_program::transfer(cpi_context, self.chest_vault.entry_fee)?;

        self.chest_vault.score_sheet.player_two = self.signer.key();
        self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_one;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, close = signer)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>Withdraw<'info> {
    pub fn creator_withdraw(
        ctx: Context<Withdraw>
    ) -> Result<()> {

        /*
            -Creator withdraws the chest reward
            -Closes the ChestVault and returns the funds to the creator
        */
        
        let chest_vault_account = &mut ctx.accounts.chest_vault;       

        // if the signer is not the creator of the chest, panic
        if ctx.accounts.signer.key() != chest_vault_account.authority {
            return Err(GameError::UnauthorizedCreator.into());
        }

        ctx.accounts.chest_vault.close(ctx.accounts.signer.to_account_info())?;

        Ok(())
    }
}