use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};
use std::time::Instant;
use std::path::Path;

use super::arena::*;
use super::player::*;

use crate::util::result::WalpurgisResult;

/// The data specific to each battle.
/// Every battle between `Player`s will be played in an `Arena`.
#[derive(Debug)]
pub struct BattleData {
    game_start: Instant,
    players: Vec<Player>,
    arena: Arena,
}

impl BattleData {
    // TODO: remove this once we don't need it anymore
    pub fn load_first_arena_and_test_player<P: AsRef<Path>>(ctx: &mut Context, asset_dir: P) -> WalpurgisResult<BattleData> {
        let asset_dir = asset_dir.as_ref();
        log::info!("Loading first arena from assets directory: `{}`", asset_dir.display());

        let arena_dir = asset_dir.join("arenas");
        Ok(BattleData {
            game_start: Instant::now(),
            arena: Arena::load_first(arena_dir)?,
            players: vec![test_player(ctx)?],
        })
    }
}

// Helpers for drawing.
impl BattleData {
    fn draw_timer(&self, ctx: &mut Context, mut param: DrawParam) -> GameResult {
        let seconds = self.game_start.elapsed().as_secs();
        let seconds = format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60);
        let timer = Text::new(seconds);
        param.dest.x += 383_f32;
        timer.draw(ctx, param)
    }
}

impl Drawable for BattleData {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.arena.draw(ctx, param)?;
        for player in &self.players {
            player.draw(ctx, param)?;
        }
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
