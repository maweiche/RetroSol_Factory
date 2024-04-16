// Functions
// 1 - admin_set_word

use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{
    ChestVault,
    GameRecord,
};

#[derive(Accounts)]
#[instruction(
    entry_fee: u64,
    secret_word: String,
    prize_pool: u64
)]
pub struct WordleInit<'info> {
    #[account(
        init,
        seeds = [
            b"chestVault",
            signer.key().as_ref(),
            Clock::get()?.unix_timestamp.to_le_bytes()
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

impl<'info>WordleInit<'info> {
    // Initializes the tipboard
    pub fn initialize_wordle(
        &mut self,
        entry_fee: u64,
        secret_word: String,
        prize_pool: u64,
    ) -> Result<()> {

        /*

        */

        self.chest_vault.set_inner(
            ChestVault {
                authority: self.signer.key(),
                created_at: Clock::get()?.unix_timestamp,
                entry_fee,
                secret_word,
                prize_pool,
                score_sheet: GameRecord {
                    game_over: false,
                    winner: None,
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
        system_program::transfer(cpi_context, prize_pool)?;

        Ok(())
    }
}