//! Structs for storing the data related to different screens within the game.

use chrono::Duration;
use crate::{
    game::{Player, Arena},
};

/// This is the data specific to each battle. Every battle between Fighters will be played in an Arena. Thus, it can look something like so:
///
/// Note that BattleData will satisfy the `ggez::Drawable` trait (requires implementing a `draw` method),
/// meaning it will be drawable to screen. It’ll likely just draw the arena and every player to screen.
#[derive(Debug)]
pub struct BattleData {
    time_since_start: Duration,
    players: Vec<Player>,
    arena: Arena,
}

/// Enum denoting the state of a particular screen. Will implement the `ggez::Drawable` trait.
#[derive(Debug)]
pub enum Screen {
    // TODO: add more screens.
    /// The state for the core gameplay screen/loop.
    Core(BattleData),
    FlashingColors,
}
impl Default for Screen {
    fn default() -> Self {
        // TODO: make the main menu the default.
        Self::FlashingColors
    }
}
