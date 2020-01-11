use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};
use ggez::nalgebra as na;
use std::time::Instant;
use std::path::Path;

use super::arena::*;
use super::player::*;
use crate::game::arena::platform::Platform;
use crate::physics::collision::*;

use crate::inputs::{HandleInput, Input};
use crate::util::{
    tuple::flip_tuple_vec,
    result::WalpurgisResult
};

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
        // Find changes.
        let mut player_changesets: Vec<Option<<Player as Collidable>::ChangeSet>> = vec![None; self.players.len()];
        let mut platform_changesets: Vec<Option<<Platform as Collidable>::ChangeSet>> = vec![None; self.arena.platforms.len()];

        let collisions = check_for_collision_pairs(self.players.as_slice(), self.arena.platforms.as_slice());
        for Collision {
            ids: (player_idx, platform_idx),
            objs: (player, platform),
            overlapping_hitboxes
        } in collisions {
            let player_changeset = player.handle_collision(platform, &overlapping_hitboxes);
            player_changesets[player_idx] = match &player_changesets[player_idx] {
                Some(changeset) => Some(changeset.merge(&player_changeset)),
                None => Some(player_changeset),
            };

            let platform_changeset = platform.handle_collision(player, &flip_tuple_vec(overlapping_hitboxes));
            platform_changesets[platform_idx] = match platform_changesets[platform_idx] {
                Some(changeset) => Some(changeset.merge(&platform_changeset)),
                None => Some(platform_changeset),
            };
        }

        let collisions = check_for_collisions(self.players.as_slice());
        for Collision {
            ids: (idx0, idx1),
            objs:(player0, player1),
            overlapping_hitboxes
        } in collisions {
            let player0_changeset = player0.handle_collision(player1, &overlapping_hitboxes);
            player_changesets[idx0] = match &player_changesets[idx0] {
                Some(changeset) => Some(changeset.merge(&player0_changeset)),
                None => Some(player0_changeset),
            };

            let player1_changeset = player1.handle_collision(player0, &flip_tuple_vec(overlapping_hitboxes));
            player_changesets[idx1] = match &player_changesets[idx1] {
                Some(changeset) => Some(changeset.merge(&player1_changeset)),
                None => Some(player1_changeset),
            };
        }

        // TODO consider rollback, generic collision resolution

        // Apply changes.
        for (idx, changeset) in player_changesets.into_iter().enumerate() {
            match changeset {
                Some(changeset) => self.players[idx].apply_changeset(changeset),
                None => (),
            };
        }
        for (idx, changeset) in platform_changesets.into_iter().enumerate() {
            match changeset {
                Some(changeset) => self.arena.platforms[idx].apply_changeset(changeset),
                None => (),
            };
        }

        // Advance time.
        for player in &mut self.players {
            player.handle_push(&self.gravity);
            player.handle_phys_update();
        }
        for platform in &mut self.arena.platforms {
            platform.handle_phys_update();
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
