use ggez::Context;
use ggez::event::{KeyCode, KeyMods};

pub type Input = (KeyCode, KeyMods);

pub trait HandleInput {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>);
}
