use anchor_lang::error_code;

#[error_code]
pub enum GameError {
    #[msg("You are not in this game!")]
    NotInGame,
    #[msg("It is not your turn!")]
    NotYourTurn,
    #[msg("Invalid pit selection!")]
    InvalidPitSelection,
    #[msg("You have not guessed the correct word yet!")]
    GameIncomplete,
    #[msg("This game has reached the max attempts!")]
    MaxAttemptsReached,
    #[msg("You are not the creator of this chest vault!")]
    UnauthorizedCreator,
}