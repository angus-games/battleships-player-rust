use crate::models::game_state::GameState;
pub struct GuessService;

impl GuessService {
    pub fn process_guess(game_state: &GameState) -> String {
        // Business logic for processing guess
        format!("Processed guess: {}", game_state)
    }
}
