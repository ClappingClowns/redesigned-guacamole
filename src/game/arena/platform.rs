use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};
use ggez::nalgebra as na;
use serde::{Serialize, Deserialize};

use crate::physics::{Collidable, BoundingBox, Effect, Collision};

/// Denotes a collidable, static section of the `Arena`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    /// `ggez`-specific. Not used for anything atm.
    #[serde(skip)]
    pub mode: Option<BlendMode>,
    /// The space occupied by the platform.
    pub body: BoundingBox,
    // TODO: Add storage for the assets' handles.
}

impl Collidable for Platform {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox] {
        self.body.get_hitboxes()
    }
    fn get_effects(&self, bb: &BoundingBox) -> Vec<Effect> {
        vec![]
    }
    fn handle_collision(&self, collision: &Collision) {}
    fn get_offset(&self) -> na::Vector2<f32> {
        na::Vector2::new(0_f32, 0_f32)
    }
}

impl Drawable for Platform {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.body.draw(ctx, param)
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
