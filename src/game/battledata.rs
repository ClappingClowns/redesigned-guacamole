use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};
use std::time::Instant;

use super::arena::*;
use super::player::*;

/// The data specific to each battle.
/// Every battle between `Player`s will be played in an `Arena`.
#[derive(Debug)]
pub struct BattleData {
    game_start: Instant,
    players: Vec<Player>,
    arena: Arena,
}

impl BattleData {
    pub fn new(arena_dir: &str) -> Result<BattleData, String> {
        Ok(BattleData {
            game_start: Instant::now(),
            arena: Arena::new(Arena::pick_first(arena_dir)?)?,
            players: vec![],
        })
    }

    fn draw_timer(&self, ctx: &mut Context, mut param: DrawParam) -> GameResult {
        let seconds = self.game_start.elapsed().as_secs();
        let seconds = format!("{}:{}", seconds / 60, seconds % 60);
        let timer = Text::new(seconds);
        param.dest.x += 400_f32;
        timer.draw(ctx, param)
    }
}

impl Drawable for BattleData {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.arena.draw(ctx, param)?;
        self.draw_timer(ctx, param)?;
        Ok(())
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
