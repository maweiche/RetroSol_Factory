use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{
    ChestVault, 
    GameRecord
};

#[derive(Accounts)]
pub struct BattleshipInit<'info> {
    #[account(
        init_if_needed,
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
    pub fn initialize_game_data(
        &mut self, 
        entry_fee: u64
    ) -> Result<()> {

        // set the game board to the initial state
        // the game board is a 2d array with 20 rows and 10 columns, all values are initialized to 0
        let game_board = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 0
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 1
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 4
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 5
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 7
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // row 9
        ];

        self.chest_vault.set_inner(
            ChestVault {
                authority: self.signer.key(),
                entry_fee,
                score_sheet: GameRecord {
                    player_one: self.signer.key(),
                    player_one_score: 0,
                    // player_two should be blank for now, will be set when the game is joined
                    player_two: None, 
                    player_two_score: 0,
                    total_moves: 0,
                    current_move: None,
                    game_over: false,
                    winner: None,
                },
                game_board,
            }
        );

        let cpi_context: CpiContext<'_, '_, '_, '_, system_program::Transfer<'_>> = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.to_account_info().clone(),
            },
        );
        system_program::transfer(cpi_context, entry_fee)?;

        Ok(())
    }
}