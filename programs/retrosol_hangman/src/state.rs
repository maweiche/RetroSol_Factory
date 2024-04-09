use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameRecord {
    pub player: Pubkey,
    pub player_score: u8,
    pub player_position: u8,
    pub incorrect_guesses: Vec<String>,
    pub correct_letters: Vec<String>,
    pub is_winner: bool,
}

#[account]
pub struct ChestVault{
    pub creator: Pubkey,
    pub chest_reward: u64,
    pub password: String,
    pub max_attempts_left: u8,
    pub entry_fee: u64,
    pub players: Vec<GameRecord>,
}

impl Space for ChestVault {
    const INIT_SPACE: usize = 8 + 32 + 8 + 64 + 1 + 8 + 324;
    // 8 for init, 32 for creator, 8 for chest_reward, 64 for password, 1 for max_attempts_left, 8 for entry_fee, 324 for players (32pubkey * 10 players + 4 for initial vec size)
}