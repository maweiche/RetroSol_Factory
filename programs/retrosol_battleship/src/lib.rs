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
}