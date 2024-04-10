use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Player already in Game.")]
    PlayerAlreadyInGame,
    #[msg("Game is full.")]
    GameIsFull,
    #[msg("Not player's turn.")]
    WrongTurn,
    #[msg("You are not the creator of the chest.")]
    NotCreator,
    #[msg("Invalid move.")]
    InvalidMove,
    #[msg("You are not the winner")]
    NotWinner,
    #[msg("Game is not over.")]
    GameIsNotOver,
    #[msg("Game is over.")]
    GameIsOver,
}