use anchor_lang::prelude::*;

mod state;
mod errors;

mod context;
use context::*;

declare_id!("FLe31QvLbzEVxNVbcudSP9msu1FFmpdj1hGn8zykvs9s");

#[program]
pub mod retrosol_mancala {
    use super::*;

    pub fn initialize_mancala(
        ctx: Context<MancalaInit>,
        entry_fee: u64,
    ) -> Result<()> {
        ctx.accounts.initialize_mancala(
            entry_fee
        )
    }

    // USER ACTIONS
    pub fn join_game(ctx: Context<PlayerJoinsGame>) -> Result<()> {
        ctx.accounts.join_game()
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Withdraw::creator_withdraw(ctx)
    }

    pub fn make_move(ctx: Context<PlayerMakesMove>, selected_pit: u8) -> Result<()> {
        ctx.accounts.make_move(selected_pit)
    }
}