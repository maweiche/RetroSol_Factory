use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameRecord {
    pub player_one: Pubkey,
    pub player_one_score: u8,
    pub player_two: Option<Pubkey>,
    pub player_two_score: u8,
    pub total_moves: u8,
    pub current_move: Option<Pubkey>,
    pub game_over: bool,
    pub winner: Option<Pubkey>,
}

impl Space for GameRecord {
    const INIT_SPACE: usize = 8 + 32 + 1 + 32 + 1 + 1 + 32 + 1 + 32;
    // 8 fort init, 32 for player_one, 1 for player_one_score, 32 for player_two, 1 for player_two_score, 1 for total_moves, 32 for current_move, 1 for game_over, 32 for winner

}

#[account]
pub struct ChestVault{
    pub authority: Pubkey,
    pub entry_fee: u64,
    pub score_sheet: GameRecord,
    pub game_board: [u8; 14],
}

impl Space for ChestVault {
    const INIT_SPACE: usize = 8 + 32 +  8 + GameRecord::INIT_SPACE + 14;
    // 8 for init, 32 for authority, 64 for password, 1 for max_attempts_left, 8 for entry_fee, 324 for players (32pubkey * 10 players + 4 for initial vec size)
}