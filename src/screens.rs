//! Structs for storing the data related to different screens within the game.
use crate::{
    game::{Player, Arena, BattleData},
};

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
