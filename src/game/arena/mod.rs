use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};

mod platform;
use platform::*;

/// Stores data for the `Arena` outside of actual players. Will satisfy the `ggez::Drawable` trait.
#[derive(Debug)]
pub struct Arena {
    mode: Option<BlendMode>,
    // background_images: Vec<ggez::Image>,
    // soundtracks: Vec<ggez::SoundData>,
    platforms: Vec<Platform>,
}

impl Default for Arena {
    fn default() -> Self {
        Arena {
            mode: None,
            platforms: vec![],
        }
    }
}

impl Drawable for Arena {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        for platform in &self.platforms {
            platform.draw(ctx, param)?;
        }
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.mode = mode;
        for platform in &mut self.platforms {
            platform.set_blend_mode(mode);
        }
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.mode
    }
}
