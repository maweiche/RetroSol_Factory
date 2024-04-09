use anchor_lang::prelude::*;
use crate::state::ChestVault;

#[derive(Accounts)]
#[instruction(
    letter: String, 
    indexes: Vec<u8>
)]
pub struct AddCorrectLetter<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info>AddCorrectLetter<'info> {
    // Initializes the tipboard
    pub fn correct_letter(
        &mut self,
        letter: String,
        indexes: Vec<u8>
    ) -> Result<()> {

        /*
            -Game adds a correct letter to the game
            -Game adds a correct letter to the correct letters vector
            -Game increments the player position
            -Game increments the player score
            -Game checks if the player has won the game
            -Game checks if the letter has already been guessed
        */

        let mut player_in_vector: bool = false;
        
        for player in self.chest_vault.players.iter() {
            if player.player == self.player.key() {
                player_in_vector = true;
            }
        }
        if player_in_vector == false {
            panic!("You have not started the game yet!");
        }
        
        let player_index = self.chest_vault.players.iter().position(|x| x.player == self.player.key()).unwrap();
        
        // if the incoming letter is already in the listed incorrect guesses vector or listed correct guesses vector, panic
        for i in 0..self.chest_vault.players[player_index].incorrect_guesses.len() {
            if self.chest_vault.players[player_index].incorrect_guesses[i] == letter {
                panic!("You have already guessed this letter incorrectly!");
            }
        }

        for i in 0..self.chest_vault.players[player_index].correct_letters.len() {
            if self.chest_vault.players[player_index].correct_letters[i] == letter {
                panic!("You have already guessed this letter correctly!");
            }
        }
        let secret_word_vector = {
            &self.chest_vault.password.split("m2w3").collect::<Vec<&str>>()
        };
        let secret_word_length = secret_word_vector.len();
        // msg!("The secret word vector is {0}", String::from(&self.chest_vault.password));
       
        for i in 0..indexes.len() {
            // msg!("Adding {0} into the correct letters vector at index {1}", letter, indexes[i]);
            
            let index_to_add = indexes[i] as usize;

            self.chest_vault.players[player_index].player_position = self.chest_vault.players[player_index].player_position + 1;
            self.chest_vault.players[player_index].player_score = self.chest_vault.players[player_index].player_position % secret_word_length as u8;        
            self.chest_vault.players[player_index].correct_letters[index_to_add] = letter.to_string();
            
            // msg!("Current player position is {0}", self.chest_vault.players[player_index].player_position);
            
            if self.chest_vault.players[player_index].player_position == secret_word_length as u8 {
            
                self.chest_vault.players[player_index].is_winner = true;
            }
            
        }
        // msg!("Correct Letters array is now {:?}", self.chest_vault.players[player_index].correct_letters);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction( letter: String )]
pub struct AddIncorrectLetter<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>AddIncorrectLetter<'info> {
    // Initializes the tipboard
    pub fn incorrect_letter(
        &mut self,
        letter: String,
    ) -> Result<()> {

        /*
            -Game adds a correct letter to the game
            -Game adds a correct letter to the correct letters vector
            -Game increments the player position
            -Game increments the player score
            -Game checks if the player has won the game
            -Game checks if the letter has already been guessed
        */

        let mut player_in_vector = false;
        
        // check if the player is in the players vector and if not, panic
        //the players vector is a vector of objects of type GameRecord, so we have to iterate over the vector and check if the player is in there
        for player in self.chest_vault.players.iter() {
            if player.player == self.player.key() {
                player_in_vector = true;
            }
        }
        if player_in_vector == false {
            panic!("You have not started the game yet!");
        }
        
        let player_index = self.chest_vault.players.iter().position(|x| x.player == self.player.key()).unwrap();
        
        // if the incoming letter is already in the listed incorrect guesses vector or listed correct guesses vector, panic
        for i in 0..self.chest_vault.players[player_index].incorrect_guesses.len() {
            if self.chest_vault.players[player_index].incorrect_guesses[i] == letter {
                panic!("You have already guessed this letter incorrectly!");
            }
        }

        for i in 0..self.chest_vault.players[player_index].correct_letters.len() {
            if self.chest_vault.players[player_index].correct_letters[i] == letter {
                panic!("You have already guessed this letter correctly!");
            }
        }
       
        self.chest_vault.players[player_index].incorrect_guesses.push(letter);
        // msg!("Incorrect Guesses array is now {:?}", self.chest_vault.players[player_index].incorrect_guesses);
      
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetChestReward<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>GetChestReward<'info> {
    // Initializes the tipboard
    pub fn issue_reward(
        ctx: Context<GetChestReward>
    ) -> Result<()> {

        let mut player_in_vector: bool = false;
        let chest_vault_account_info: &AccountInfo<'_> = &ctx.accounts.chest_vault.to_account_info();
        let chest_vault_account: &mut ChestVault = &mut *ctx.accounts.chest_vault;

        for player in chest_vault_account.players.iter() {
            if player.player == ctx.accounts.player.key() {
                player_in_vector = true;
            }
        }
        if player_in_vector == false {
            panic!("You have not started the game yet!");
        }
        
        let chest_reward = chest_vault_account.chest_reward;
        let player_index = chest_vault_account.players.iter().position(|x| x.player == ctx.accounts.player.key()).unwrap();

        if chest_vault_account.players[player_index].is_winner == false {
            panic!("You have not solved the puzzle yet!");
        }

        let secret_word_vector = {
            &chest_vault_account.password.split("m2w3").collect::<Vec<&str>>()
        };
        let secret_word_length = secret_word_vector.len();
        if chest_vault_account.players[player_index].player_position == secret_word_length as u8 {
            
            // issue transfer of chest reward to player
            **chest_vault_account_info.try_borrow_mut_lamports()? -= chest_reward;
            **ctx
                .accounts
                .player
                .to_account_info()
                .try_borrow_mut_lamports()? += chest_reward;

            chest_vault_account.max_attempts_left = 0;
        }

        Ok(())
    }
}
