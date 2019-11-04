use chrono::Duration;
use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};
use ron::de::from_reader;
use std::fs::File;

use super::arena::*;
use super::player::*;

/// This is the data specific to each battle. Every battle between Fighters will be played in an Arena.
///
/// Note that BattleData will satisfy the `ggez::Drawable` trait (requires implementing a `draw` method),
/// meaning it will be drawable to screen. It’ll likely just draw the arena and every player to screen.
#[derive(Debug)]
pub struct BattleData {
    time_since_start: Duration,
    players: Vec<Player>,
    arena: Arena,
}

impl Default for BattleData {
    fn default() -> Self {
        BattleData {
            time_since_start: Duration::zero(),
            players: vec![],
            arena: Arena::default(),
        }
    }
}

impl Drawable for BattleData {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.arena.draw(ctx, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.arena.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.arena.blend_mode()
    }
}


pub struct BattleDataBuilder {
    pub arena_dir: String,
    pub players: Vec<Player>,
}

impl BattleDataBuilder {
    pub fn build(self) -> Result<BattleData, String> {
        let f = File::open(&self.arena_dir).map_err(|err| err.to_string())?;
        let arena: Arena = from_reader(f).map_err(|err| err.to_string())?;

        Ok(BattleData {
            time_since_start: Duration::zero(),
            arena: arena,
            players: self.players,
        })
    }
}
