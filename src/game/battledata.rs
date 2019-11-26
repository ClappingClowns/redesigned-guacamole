use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};
use ggez::nalgebra as na;
use std::time::Instant;
use std::path::Path;

use super::arena::*;
use super::player::*;
use crate::physics::collision::*;

use crate::inputs::{HandleInput, Input};
use crate::util::result::WalpurgisResult;

/// The data specific to each battle.
/// Every battle between `Player`s will be played in an `Arena`.
#[derive(Debug)]
pub struct BattleData {
    game_start: Instant,
    players: Vec<Player>,
    arena: Arena,
    gravity: na::Vector2<f32>,
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
            gravity: na::Vector2::<f32>::new(0.0, 0.01),
        })
    }
}

impl HandleInput for BattleData {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) {
        for player in &mut self.players {
            player.handle_input(ctx, fire_once_key_buffer);
        }
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

    pub fn handle_update(&mut self) {
        let collisions = check_for_collision_pairs(self.players.as_slice(), self.arena.platforms.as_slice());
        for Collision {
            ids: (id0, id1),
            objs:(obj0, obj1),
            overlapping_hitboxes
        } in collisions {
            obj0.handle_collision(obj1, &overlapping_hitboxes);
            let x: Vec<_> = overlapping_hitboxes.into_iter().map(|(b0, b1)| (b1, b0)).collect();
            obj1.handle_collision(obj0, &x);
        }
        for player in &mut self.players {
            player.handle_push(&self.gravity);
            player.handle_phys_update();
        }
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
