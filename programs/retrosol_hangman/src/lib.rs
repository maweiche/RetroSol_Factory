use anchor_lang::prelude::*;

mod state;
// mod errors;
// mod constant;

mod context;
use context::*;

declare_id!("4hvTWxjk5UPGhpVuF1tM9USHLgzKon2W2mrgfxitnEvs");

#[program]
pub mod retrosol_hangman {
    use super::*;

    pub fn initialize_hangman(
        ctx: Context<HangmanInit>,
        chest_reward: u64,
        password: String,
        max_attempts: u8,
        entry_fee: u64
    ) -> Result<()> {
        ctx.accounts.initialize_hangman(
            chest_reward,
            password,
            max_attempts,
            entry_fee
        )
    }

    // USER ACTIONS
    pub fn start_game(ctx: Context<PlayerStartsGame>) -> Result<()> {
        ctx.accounts.start_game()
    }

    pub fn creator_withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Withdraw::creator_withdraw(ctx)
    }

    // GAME ACTIONS
    pub fn correct_letter(
        ctx: Context<AddCorrectLetter>,
        letter: String, 
        indexes: Vec<u8>
    ) -> Result<()> {
        ctx.accounts.correct_letter(
            letter,
            indexes
        )
    }

    pub fn incorrect_letter(
        ctx: Context<AddIncorrectLetter>,
        letter: String
    ) -> Result<()> {
        ctx.accounts.incorrect_letter(
            letter
        )
    }

    pub fn issue_reward(ctx: Context<GetChestReward>) -> Result<()> {
        GetChestReward::issue_reward(ctx)
    }
}