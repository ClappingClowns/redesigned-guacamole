use ggez::{Context, GameResult};
use ggez::graphics::{self, Drawable, DrawParam, Rect, BlendMode, Mesh, DrawMode};
use serde::{Serialize, Deserialize};

use crate::physics::{Collidable, BoundingBox, Effect, Collision};

/// Denotes a collidable, static section of the `Arena`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    // `ggez`-specific. Not used for anything atm.
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
}

impl Drawable for Platform {
    fn draw(&self, ctx: &mut Context, mut param: DrawParam) -> GameResult {
        let rect = Rect::new(0f32, 0f32, 1.0, 1.0);

        let body = &self.body;
        param.rotation += body.ori;
        param.scale.x *= body.size[0];
        param.scale.y *= body.size[1];
        param.dest.x += body.pos[0];
        param.dest.y += body.pos[1];

        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &mesh, param)
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
