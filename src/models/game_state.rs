use std::fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Coordinate(pub [i32; 2]);

#[derive(Deserialize, Debug)]
pub struct Pair(pub Coordinate, pub Coordinate);

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hit: Vec<Coordinate>,
    pub miss: Vec<Coordinate>,
    pub sunk: Vec<[Coordinate; 2]>,
}

#[derive(Deserialize, Debug)]
pub struct History {
    pub player1: Player,
    pub player2: Player,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub state: String, // "playing", "won", or "starting"
    pub player: String, // "player1" or "player2"
    pub reason: Option<String>, // Optional reason for the game's end
}

#[derive(Deserialize, Debug)]
pub struct GameState {
    pub status: Status,
    pub history: History,
    pub limits: [Coordinate; 2], // Two coordinates for the game board limits
    pub ship_lengths: Vec<u32>, // Array of ship lengths
}

// implement display for GameState
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0[0], self.0[1])
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ship Lengths: {:?}\nLimits: {:?}",
               self.ship_lengths, self.limits)
    }
}
