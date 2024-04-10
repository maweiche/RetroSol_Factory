use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::{
    state::ChestVault,
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

#[derive(Accounts)]
#[instruction( selected_pit: u8 )]
pub struct PlayerMakesMove<'info> {
    #[account(mut)]
    pub chest_vault: Account<'info, ChestVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info>PlayerMakesMove<'info> {
    pub fn make_move(
        &mut self,
        selected_pit: u8
    ) -> Result<()> {

        /*
        
        */
        
        if selected_pit == 0 || selected_pit == 7 {
            return Err(GameError::InvalidPitSelection.into());
        }

        // if the player is not one of the players in the game, then panic
        // if the player is not the current player, then panic
        
        if self.chest_vault.score_sheet.player_one != self.signer.key() && self.chest_vault.score_sheet.player_two != self.signer.key() {
            return Err(GameError::NotInGame.into());
        }

        if self.chest_vault.score_sheet.current_move != self.signer.key() {
            return Err(GameError::NotYourTurn.into());
        }

        // create a variable to hold the current player moving, if it is player one, then it is 1, if it is player two, then it is 2
        let current_player_moving: u8;

        if self.chest_vault.score_sheet.current_move == self.chest_vault.score_sheet.player_one {
            current_player_moving = 1;
        } else {
            current_player_moving = 2;
        }

        if self.signer.key() == self.chest_vault.score_sheet.player_one {
            // if the selected pit is not 1-6, then panic
            if selected_pit < 1 || selected_pit > 6 {
                panic!("Invalid pit selection");
            }
        } else {
            // if the selected pit is not 8-13, then panic
            if selected_pit < 8 || selected_pit > 13 {
                panic!("Invalid pit selection");
            }
        }

        // when the player moves, they select a pit to move from
        //the player's pieces are then moved counter-clockwise around the board, one piece per pit
        // if the last piece lands in the player's score pit, they get another turn
        // if the last piece lands in an empty pit on their side, they capture that piece and all the pieces in the pit directly across from it and put them in their score pit
       // 0 is player 2's score pit and 7 is player 1's score pit
        // if the last piece lands in an empty pit on their side, they capture that piece and all the pieces in the pit directly across from it and put them in their score pit, if the pit directly across from it is empty, then nothing happens

        // ///////Game Board///////////////
        // Player 2///////////////////////
        // // 13 12 11 10 9  8 //////////
        // 0 ////////////////// 7 //////
        // // 1  2  3  4  5  6 ////////
        // Player 1///////////////////

        // if the selected pit is empty, then panic
        if self.chest_vault.game_board[selected_pit as usize] == 0 {
            panic!("Selected pit is empty");
        }

        // get the number of pieces in the selected pit
        let mut pieces_in_selected_pit = self.chest_vault.game_board[selected_pit as usize];

        // set the selected pit to 0
        self.chest_vault.game_board[selected_pit as usize] = 0;

        // move the pieces counter-clockwise around the board, one piece per pit
        let mut current_pit = selected_pit + 1;
        // while there are still pieces to move
        while pieces_in_selected_pit > 0 {

            // if the current pit is 14, then set it to 0
            if current_pit == 14 {
                current_pit = 0;
            }

            // if the current pit is the opponent's score pit, then skip it
            if current_pit == 0 && current_player_moving == 1 {
                current_pit += 1;
            } else if current_pit == 7 && current_player_moving == 2 {
                current_pit += 1;
            }

            // increment the number of pieces in the current pit
            self.chest_vault.game_board[current_pit as usize] += 1;

            // decrement the number of pieces left to move
            pieces_in_selected_pit -= 1;

            // increment the current pit
            current_pit += 1;
            
        }
        let last_pit_landed_in = current_pit - 1;
        // if the last piece lands in an empty pit on their side, they capture the piece in the empty and all the pieces in the pit directly across from it and put them in their score pit (playerOne score pit = 7, playerTwo score pit = 0)
        // if the pit directly across from it is empty, then nothing happens
        // 1 matches with 13, 2 matches with 12, 3 matches with 11, 4 matches with 10, 5 matches with 9, 6 matches with 8
        if last_pit_landed_in != 0 && last_pit_landed_in != 7 {
            if current_player_moving == 1 {
                if self.chest_vault.game_board[last_pit_landed_in as usize] == 1 && self.chest_vault.game_board[14 - last_pit_landed_in as usize] != 0 && last_pit_landed_in < 7 && last_pit_landed_in > 0 {
                    self.chest_vault.game_board[7] += self.chest_vault.game_board[14 - last_pit_landed_in as usize] + 1;
                    self.chest_vault.game_board[last_pit_landed_in as usize] = 0;
                    self.chest_vault.game_board[14 - last_pit_landed_in as usize] = 0;
                }
            } else {
                if self.chest_vault.game_board[last_pit_landed_in as usize] == 1 && self.chest_vault.game_board[14 - last_pit_landed_in as usize] != 0 && last_pit_landed_in < 14 && last_pit_landed_in > 7 {
                    self.chest_vault.game_board[0] += self.chest_vault.game_board[14 - last_pit_landed_in as usize] + 1;
                    self.chest_vault.game_board[last_pit_landed_in as usize] = 0;
                    self.chest_vault.game_board[14 - last_pit_landed_in as usize] = 0;
                }
            }
        }

        // if the current_player_moving is 1 and there are no pieces in pits 1-6, then the game is over
        // if the current_player_moving is 2 and there are no pieces in pits 8-13, then the game is over
        if current_player_moving == 1 {
            if self.chest_vault.game_board[1] == 0 && self.chest_vault.game_board[2] == 0 && self.chest_vault.game_board[3] == 0 && self.chest_vault.game_board[4] == 0 && self.chest_vault.game_board[5] == 0 && self.chest_vault.game_board[6] == 0 {
                self.chest_vault.score_sheet.game_over = true;
                if self.chest_vault.game_board[7] > self.chest_vault.game_board[0] {
                    self.chest_vault.score_sheet.winner = self.chest_vault.score_sheet.player_one;
                } else if self.chest_vault.game_board[7] < self.chest_vault.game_board[0] {
                    self.chest_vault.score_sheet.winner = self.chest_vault.score_sheet.player_two;
                } else {
                    self.chest_vault.score_sheet.winner = Pubkey::default();
                }
            }
        } else if current_player_moving == 2{
            if self.chest_vault.game_board[8] == 0 && self.chest_vault.game_board[9] == 0 && self.chest_vault.game_board[10] == 0 && self.chest_vault.game_board[11] == 0 && self.chest_vault.game_board[12] == 0 && self.chest_vault.game_board[13] == 0 {
                self.chest_vault.score_sheet.game_over = true;
                if self.chest_vault.game_board[7] > self.chest_vault.game_board[0] {
                    self.chest_vault.score_sheet.winner = self.chest_vault.score_sheet.player_one;
                } else if self.chest_vault.game_board[7] < self.chest_vault.game_board[0] {
                    self.chest_vault.score_sheet.winner = self.chest_vault.score_sheet.player_two;
                } else {
                    self.chest_vault.score_sheet.winner = Pubkey::default();
                }
            }
        }
        
        // if the last piece lands in the player's score pit, they get another turn, else it is the other player's turn
        if last_pit_landed_in == 7 && current_player_moving == 1 {
            self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_one;
        } else if last_pit_landed_in == 0 && current_player_moving == 2 {
            self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_two;
        } else {
            if current_player_moving == 1 {
                self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_two;
            } else {
                self.chest_vault.score_sheet.current_move = self.chest_vault.score_sheet.player_one;
            }
        }

        // update the score_sheet
        self.chest_vault.score_sheet.player_one_score = self.chest_vault.game_board[7];
        self.chest_vault.score_sheet.player_two_score = self.chest_vault.game_board[0];
        self.chest_vault.score_sheet.total_moves += 1;


        Ok(())
    }
}