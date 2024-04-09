use anchor_lang::prelude::*;

declare_id!("FmV6Zwmk8sz8SbxJC7t7SbsfUKPJ3xL7hHirjgE9CMpR");

#[program]
pub mod retrosol_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
