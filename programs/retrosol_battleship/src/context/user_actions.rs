use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::errors::ErrorCode;
use crate::state::ChestVault;

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

#[derive(Accounts)]
#[instruction(selected_squares: [[u8; 10]; 5])]
pub struct PlayerPlacement<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerPlacement<'info> {
    pub fn choose_placement(
        &mut self,
        selected_squares: [[u8; 10]; 5]
    ) -> Result<()> {
        
        // if the game is already over, then panic
        if self.chest_vault.score_sheet.game_over == true {
            return Err(ErrorCode::GameIsOver.into());
        }

        if self.signer.key() == self.chest_vault.score_sheet.player_one {
            //  the seclected squares will be the first 5 rows of the game board
            //  replace the game board's rows 0-4 with the selected squares
            //  ex. of selected squares:
            // [
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 3], // row 0
            //     [1, 0, 4, 0, 0, 0, 0, 0, 0, 3], // row 1
            //     [1, 0, 4, 0, 2, 2, 2, 2, 2, 0], // row 2
            //     [0, 0, 4, 0, 0, 0, 0, 0, 0, 0], // row 3
            //     [0, 0, 4, 0, 0, 0, 0, 0, 0, 0], // row 4
            // ]
            // iterate through the selected squares and replace the game board's rows 0-4
            let mut index = 0;
            for row in selected_squares.iter() {
                
                let replacement_row: [u8; 10] = row.clone();
                
                // replace the game board's row with the replacement row
                // ex. of replacement row:
                // [1, 0, 0, 0, 0, 0, 0, 0, 0, 3] // row 0
                // [1, 0, 4, 0, 0, 0, 0, 0, 0, 3] // row 1
                // [1, 0, 4, 0, 2, 2, 2, 2, 2, 0] // row 2
                // [0, 0, 4, 0, 0, 0, 0, 0, 0, 0] // row 3
                // [0, 0, 4, 0, 0, 0, 0, 0, 0, 0] // row 4
                //  ...
                self.chest_vault.game_board[index] = replacement_row;
                index += 1;
            }   
        } else if Some(self.signer.key()) == self.chest_vault.score_sheet.player_two {
            // replace the game boards rows 5-9
            let mut second_player_index = 5;
            for row in selected_squares.iter() {
                
                let replacement_row: [u8; 10] = row.clone();
                
                // replace the game board's row with the replacement row
                self.chest_vault.game_board[second_player_index] = replacement_row;

                second_player_index += 1;
            }
        }

        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(selected_square: [u8; 2])]
pub struct PlayerAttacks<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerAttacks<'info> {
    pub fn attack(
        &mut self,
        selected_square: [u8; 2]
    ) -> Result<()> {
        
        // if the game is already over, then panic
        if self.chest_vault.score_sheet.game_over == true {
            return Err(ErrorCode::GameIsOver.into());
        }

        if self.chest_vault.score_sheet.current_move != Some(self.signer.key()) {
            return Err(ErrorCode::WrongTurn.into());
        }

        let square_to_attack_row = selected_square[0]; 
        let square_to_attack_column = selected_square[1];
        
        // check if the square_to_attack is a valid square, if it's a 7, 8, or 9, then it's not a valid square
        if self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] == 7 || self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] == 8 || self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] == 9 {
            return Err(ErrorCode::InvalidMove.into());
        }
       
        // change the game board's square to 9 if it is a hit
        if self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] != 0  && self.signer.key() == self.chest_vault.score_sheet.player_one {
            self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] = 9;
        } else if self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] != 0  && Some(self.signer.key()) == self.chest_vault.score_sheet.player_two{
            // change the game board's square to 8 if it is a miss
            self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] = 8;
        } else {
            self.chest_vault.game_board[square_to_attack_row as usize][square_to_attack_column as usize] = 7;
        }

        // change the current_move to the other player
        if self.chest_vault.score_sheet.current_move == Some(self.chest_vault.score_sheet.player_one) {
            self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_two;
        } else {
            self.chest_vault.score_sheet.current_move = Some(self.chest_vault.score_sheet.player_one);
        }

        // check if the game is over
        // if rows 0-4 are all 9s, 0s, and 8s, then player two wins
        // if rows 5-9 are all 9s, 0s, and 8s, then player one wins
        let mut player_one_wins = false;
        let mut player_two_wins = false;
        let mut player_one_squares = 0;
        let mut player_two_squares = 0;
        for row in self.chest_vault.game_board.iter() {
            for square in row.iter() {
                if *square == 9 {
                    player_one_squares += 1;
                } else if *square == 8 {
                    player_two_squares += 1;
                }
            }
        }

        if player_one_squares == 14 {
            player_one_wins = true;
        } else if player_two_squares == 14 {
            player_two_wins = true;
        }

        if player_one_wins == true {
            self.chest_vault.score_sheet.game_over = true;
            self.chest_vault.score_sheet.winner = Some(self.chest_vault.score_sheet.player_one);
        } else if player_two_wins == true {
            self.chest_vault.score_sheet.game_over = true;
            self.chest_vault.score_sheet.winner = self.chest_vault.score_sheet.player_two;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawLoot<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>WithdrawLoot<'info> {
    pub fn withdraw(
        &mut self,
    ) -> Result<()> {
        
        // if the signer is not the creator of the chest, panic
        if Some(self.signer.key()) != self.chest_vault.score_sheet.winner {
            return Err(ErrorCode::NotWinner.into());
        }
        
        let chest_reward = self.chest_vault.entry_fee * 2;
        // issue transfer of chest_reward to the winner from the chest_vault_account
        **self.chest_vault.to_account_info().try_borrow_mut_lamports()? -= chest_reward;
            **self
                .signer
                .to_account_info()
                .try_borrow_mut_lamports()? += chest_reward;

        Ok(())
    }
}