use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{
    ChestVault,
    GameRecord,
};

#[derive(Accounts)]
#[instruction(
    entry_fee: u64
)]
pub struct MancalaInit<'info> {
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

impl<'info>MancalaInit<'info> {
    // Initializes the tipboard
    pub fn initialize_mancala(
        &mut self,
        entry_fee: u64
    ) -> Result<()> {

        /*
            -set the game board to the initial state
            -the game board is an array of 0-13, where 0 is player 1's score pit and 1-6 holds player 1's pieces, 7 is player 2's score pit and 8-13 holds player 2's pieces
            -the game board is initialized to [0,4,4,4,4,4,4,0,4,4,4,4,4,4]
            -set the creator of the game to the signer so they can close it if no one joins
            -set the entry fee for the game
            -set the score sheet to the initial state
            -transfer the entry fee to the chest vault
        */

        self.chest_vault.set_inner(
            ChestVault {
                authority: self.signer.key(),
                game_board: [0,4,4,4,4,4,4,0,4,4,4,4,4,4],
                entry_fee,
                score_sheet: GameRecord {
                    player_one: self.signer.key(),
                    player_one_score: 0,
                    // player_two should be blank for now, will be set when the game is joined
                    player_two: Pubkey::new_from_array([0u8; 32]), 
                    player_two_score: 0,
                    total_moves: 0,
                    current_move: Pubkey::new_from_array([0u8; 32]),
                    game_over: false,
                    winner: Pubkey::new_from_array([0u8; 32]),
                },
            }
        );

        let cpi_context: CpiContext<'_, '_, '_, '_, system_program::Transfer<'_>> = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.chest_vault.to_account_info().clone(),
            },
        );
        msg!("Transfering entry fee to chest vault account {0}", entry_fee);
        system_program::transfer(cpi_context, entry_fee)?;

        Ok(())
    }
}