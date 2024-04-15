use anchor_lang::prelude::*;
mod state;
mod errors;

mod context;
use context::*;

declare_id!("FmV6Zwmk8sz8SbxJC7t7SbsfUKPJ3xL7hHirjgE9CMpR");

#[program]
pub mod retrosol_battleship {
    use super::*;

    pub fn initialize_battleship(
        ctx: Context<BattleshipInit>,
        entry_fee: u64,
    ) -> Result<()> {
        ctx.accounts.initialize_battleship(
            entry_fee
        )
    }

    pub fn join_game (
        ctx: Context<PlayerJoinsGame>,
    ) -> Result<()> {
        ctx.accounts.join_game()
    }

    pub fn choose_placement(
        ctx: Context<PlayerPlacement>,
        selected_squares: [[u8; 10]; 5]
    ) -> Result<()> {
        ctx.accounts.choose_placement(selected_squares)
    }

    pub fn attack(
        ctx: Context<PlayerAttacks>,
        selected_square: [u8; 2]
    ) -> Result<()> {
        ctx.accounts.attack(selected_square)
    }

    pub fn withdraw(
        ctx: Context<WithdrawLoot>,
    ) -> Result<()> {
        ctx.accounts.withdraw()
    }

    pub fn close_game(
        ctx: Context<CloseAccount>,
    ) -> Result<()> {
        ctx.accounts.close_game()
    }
}