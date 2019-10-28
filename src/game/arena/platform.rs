use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};

use crate::{physics};

/// Denotes a static section of the `Arena`. Implements `ggez::Drawable`.
#[derive(Debug)]
pub struct Platform {
    mode: Option<BlendMode>,
    /// The portion occupied by the platform.
    body: [physics::BoundingBox; 1],
    /// If a player is allowed to move through the platform.
    can_move_through: bool,
    // TODO: Add storage for the assets' handles.
}

impl physics::Collidable for Platform {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[physics::BoundingBox] {
        &self.body
    }
    fn get_effects(&self) {}
}

impl Drawable for Platform {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.mode = mode;
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.mode
    }
}

