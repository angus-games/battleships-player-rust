use crate::models::game_state::GameState;
pub struct PositionService;

impl PositionService{
    pub fn process_position(game_state: &GameState) -> String {
        // Business logic for processing position
        format!("Processed position: {}", game_state)
    }
}
