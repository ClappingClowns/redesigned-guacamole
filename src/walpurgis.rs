use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Drawable, DrawParam};

use crate::{
    screens,
    settings,
    inputs::{HandleInput, Input},
    util::result::WalpurgisResult,
};

/// This is the global game state.
pub struct Walpurgis {
    // TODO: Some shared state.
    /// Screen specific state.
    screen: screens::Screen,
    fire_once_key_buffer: Vec<Input>,
}

impl Walpurgis {
    /// Create a new game state.
    pub fn new(ctx: &mut Context, assets: &settings::Assets) -> WalpurgisResult<Self> {
        // Load/create resources here: images, fonts, sounds, etc.
        Ok(Walpurgis {
            screen: screens::Screen::first_battle(ctx, assets)?,
            fire_once_key_buffer: vec![],
        })
    }
}

impl EventHandler for Walpurgis {
    /// This executes a tick update.
    /// 1. Collision detection
    /// 2. Platform/Floor Collision
    /// 3. Input management
    /// 4. Update components
    ///     * Players
    ///     * Arena
    /// 5. Re-render
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, 60) {
            self.screen.handle_input(ctx, &self.fire_once_key_buffer);
            self.fire_once_key_buffer.clear();

            self.screen.handle_update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context)-> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.screen.draw(ctx, DrawParam::new())?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }
        match key {
            KeyCode::Escape => {
                log::info!("Escape pressed. Stopping game loop.");
                event::quit(ctx);
            }
            key => {
                self.fire_once_key_buffer.push((key, mods));
            }
        }
    }
}
