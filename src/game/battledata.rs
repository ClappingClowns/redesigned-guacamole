use chrono::Duration;
use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};

use super::arena::*;
use super::player::*;

/// The data specific to each battle.
/// Every battle between `Player`s will be played in an `Arena`.
#[derive(Debug)]
pub struct BattleData {
    time_since_start: Duration,
    players: Vec<Player>,
    arena: Arena,
}

impl BattleData {
    pub fn new(arena_dir: &str) -> Result<BattleData, String> {
        Ok(BattleData {
            time_since_start: Duration::zero(),
            arena: Arena::new(Arena::pick_first(arena_dir)?)?,
            players: vec![],
        })
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
