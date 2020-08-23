//! Structs for storing the data related to different screens within the game.
use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};

use crate::{
    settings,
    inputs::{HandleInput, Input},
    util::result::WalpurgisResult,
};

mod battle;
use self::battle::BattleData;
mod mainmenu;
use self::mainmenu::MainMenuData;

/// Enum denoting the state of a particular screen. Will implement the `ggez::Drawable` trait.
#[derive(Debug)]
pub enum Screen {
    // TODO: add more screens.
    /// The state for the core gameplay screen/loop.
    Battle(BattleData),
    /// Main menu for game.
    MainMenu(MainMenuData),
}

impl HandleInput for Screen {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) {
        match self {
            Self::Battle(data) => data.handle_input(ctx, fire_once_key_buffer),
            Self::MainMenu(data) => data.handle_input(ctx, fire_once_key_buffer),
        }
    }
}

impl Screen {
    pub fn handle_update(&mut self) {
        match self {
            Self::Battle(data) => data.handle_update(),
            Self::MainMenu(data) => data.handle_update(),
        }
    }

    pub fn first_battle(ctx: &mut Context, assets: &settings::Assets) -> WalpurgisResult<Self> {
        Ok(Self::Battle(battle::BattleData::load_first_arena_and_test_player(ctx, &assets.root)?))
    }
}

impl Drawable for Screen {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        match self {
            Self::Battle(data) => data.draw(ctx, param),
            Self::MainMenu(data) => data.draw(ctx, param),
        }
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        match self {
            Self::Battle(battle_data) => battle_data.dimensions(ctx),
            Self::MainMenu(data) => data.dimensions(ctx),
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        match self {
            Self::Battle(battle_data) => battle_data.set_blend_mode(mode),
            Self::MainMenu(data) => data.set_blend_mode(mode),
        }
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        match self {
            Self::Battle(battle_data) => battle_data.blend_mode(),
            Self::MainMenu(data) => data.blend_mode(),
        }
    }
}
