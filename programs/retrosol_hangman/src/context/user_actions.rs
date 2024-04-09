use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{
    ChestVault,
    GameRecord,
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

        if self.chest_vault.max_attempts_left == 0 {
            panic!("Max attempts reached. Game Over!");
        }

        for player in self.chest_vault.players.iter() {
            if player.player == self.signer.key() && player.player_score > 0 {
                panic!("You have already played and your score is {0}. You can not play again!", player.player_score);
            }
        }

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.chest_vault.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, self.chest_vault.entry_fee)?;

        let secret_word_vector = {
            &self.chest_vault.password.split("m2w3").collect::<Vec<&str>>()
        };
        let secret_word_length = secret_word_vector.len();

        let item = GameRecord {
            player: self.signer.key(),
            player_score: 0,
            player_position: 0,
            incorrect_guesses: vec![],
            correct_letters: vec!["_".to_string(); secret_word_length],
            is_winner: false,
        };

        self.chest_vault.players.push(item);

        self.chest_vault.max_attempts_left = self.chest_vault.max_attempts_left - 1;

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
        
        let authority = chest_vault_account.creator;            

        // if the signer is not the creator of the chest, panic
        if ctx.accounts.signer.key() != authority {
            panic!("You are not the creator of the chest, you can not withdraw!");
        }

        ctx.accounts.chest_vault.close(ctx.accounts.signer.to_account_info())?;

        Ok(())
    }
}