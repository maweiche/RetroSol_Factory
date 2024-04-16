use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameRecord {
    pub game_over: bool,
    pub winner: Option<Pubkey>,
}

impl Space for GameRecord {
    const INIT_SPACE: usize = 8 + 1 + 32;
}

#[account]
pub struct ChestVault{
    pub authority: Pubkey,
    pub created_at: i64,
    pub entry_fee: u64,
    pub prize_pool: u64,
    pub score_sheet: GameRecord,
    pub secret_word: String
}

impl Space for ChestVault {
    const INIT_SPACE: usize = 8 + 32 +  8 + GameRecord::INIT_SPACE + 64;
}