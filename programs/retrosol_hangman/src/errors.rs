use anchor_lang::error_code;

#[error_code]
pub enum GameError {
    #[msg("You have not started the game yet!")]
    NotInGame,
    #[msg("You have already guessed this letter incorrectly!")]
    RepeatIncorrectGuess,
    #[msg("You have alreayd guessed this letter correctly!")]
    RepeatCorrectGuess,
    #[msg("You have not guessed the correct word yet!")]
    GameIncomplete,
    #[msg("This game has reached the max attempts!")]
    MaxAttemptsReached,
    #[msg("You are not the creator of this chest vault!")]
    UnauthorizedCreator,
}
