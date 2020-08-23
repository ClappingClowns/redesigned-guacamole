//! A collection of structs useful for drawing and managing the core gameplay loop.
//!
//! ## Rendering Details
//! Overlapping Attacks
//! If Player A launches an attack and so does Player B, their attacks could overlap. If their attacks overlap, which attack appears on top?
mod arena;
mod platform;
mod player;
mod interactions;

use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};
use ggez::nalgebra as na;
use std::time::Instant;
use std::path::Path;

use crate::{
    util::{
        result::WalpurgisResult
    },
    screens::battle::{
        arena::Arena,
        platform::Platform,
        player::{Player, Changes as PlayerChangeSet, test_player},
    },
    inputs::{HandleInput, Input},
    physics::collision::*,
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
        use interactions as res;

        // Find changes.
        let grav_changeset = PlayerChangeSet {
            force: self.gravity,
            ..Default::default()
        };
        let mut player_changesets: Vec<<Player as Collidable>::ChangeSet>
            = vec![grav_changeset; self.players.len()];
        let mut platform_changesets: Vec<Option<<Platform as Collidable>::ChangeSet>>
            = vec![None; self.arena.platforms.len()];

        let collisions = check_for_collision_pairs(self.players.as_slice(), self.arena.platforms.as_slice());
        for c in collisions {
            let (player_id, platform_id) = c.ids;
            let (player_changeset, platform_changeset) = res::handle_player_platform_collision(c);
            if let Some(player_changeset) = player_changeset {
                player_changesets[player_id]
                    = player_changesets[player_id].merge(&player_changeset);
            }
            if let Some(platform_changeset) = platform_changeset {
                platform_changesets[platform_id] = match platform_changesets[platform_id] {
                    Some(changeset) => Some(changeset.merge(&platform_changeset)),
                    None => Some(platform_changeset),
                };
            }
        }

        let collisions = check_for_collisions(self.players.as_slice());
        for c in collisions {
            let (p0_id, p1_id) = c.ids;
            let (changeset0, changeset1) = res::handle_player_player_collision(c);
            if let Some(changeset0) = changeset0 {
                player_changesets[p0_id]
                    = player_changesets[p0_id].merge(&changeset0);
            }
            if let Some(changeset1) = changeset1 {
                player_changesets[p1_id]
                    = player_changesets[p1_id].merge(&changeset1);
            }
        }

        // TODO consider rollback, generic collision resolution

        // Apply changes.
        for (idx, changeset) in player_changesets.into_iter().enumerate() {
            self.players[idx].apply_changeset(changeset);
        }
        for (idx, changeset) in platform_changesets.into_iter().enumerate() {
            match changeset {
                Some(changeset) => self.arena.platforms[idx].apply_changeset(changeset),
                None => (),
            };
        }

        // Advance time.
        for player in &mut self.players {
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
