use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, Drawable, DrawParam};

use super::screens;

/// This is the global game state. We are likely going to want to have a couple of menu screens and a game screen. Thus, in Rust, it could look like so:
pub struct Walpurgis {
    // TODO: Some shared state
    /// Screen specific state.
    screen: screens::Screen,
}

/// Create a new game state, referencing the provided `ggez` `Context`.
pub fn new(_ctx: &mut Context) -> Walpurgis {
    // Load/create resources here: images, fonts, sounds, etc.
    Walpurgis {
        screen: screens::Screen::default(),
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
        // Update code here...
        while ggez::timer::check_update_time(ctx, 60) {
            // TODO: useful work.
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context)-> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.screen.draw(ctx, DrawParam::new())?;
        graphics::present(ctx)
    }
}
