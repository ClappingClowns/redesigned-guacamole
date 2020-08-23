use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, Text, BlendMode};

use crate::inputs::{HandleInput, Input};

#[derive(Debug)]
pub struct MainMenuData {
    /// `ggez`-specific. Not really used for anything atm.
    mode: Option<BlendMode>,
}
impl MainMenuData {
    pub fn handle_update(&mut self) {
    }
}
impl Drawable for MainMenuData {
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
impl HandleInput for MainMenuData {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) {
    }
}

