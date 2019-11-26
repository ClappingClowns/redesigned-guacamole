//! Structs for storing the data related to different screens within the game.
use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};

use crate::game::{BattleData};
use crate::inputs::{HandleInput, Input};

/// Enum denoting the state of a particular screen. Will implement the `ggez::Drawable` trait.
#[derive(Debug)]
pub enum Screen {
    // TODO: add more screens.
    /// The state for the core gameplay screen/loop.
    Core(BattleData),
}

impl HandleInput for Screen {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) {
        match self {
            Self::Core(battle_data) => battle_data.handle_input(ctx, fire_once_key_buffer),
        }
    }
}

impl Screen {
    pub fn handle_update(&mut self) {
        match self {
            Self::Core(battle_data) => battle_data.handle_update(),
        }
    }
}

impl Drawable for Screen {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        match self {
            Self::Core(battle_data) => battle_data.draw(ctx, param)
        }
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        match self {
            Self::Core(battle_data) => battle_data.dimensions(ctx)
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        match self {
            Self::Core(battle_data) => battle_data.set_blend_mode(mode)
        }
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        match self {
            Self::Core(battle_data) => battle_data.blend_mode()
        }
    }
}
