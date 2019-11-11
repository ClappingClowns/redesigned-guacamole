use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::graphics::{self, Drawable, DrawParam};

use crate::{screens, settings};
use crate::game::BattleData;
use crate::util::result::WalpurgisResult;

/// This is the global game state.
pub struct Walpurgis {
    // TODO: Some shared state.
    /// Screen specific state.
    screen: screens::Screen,
}

impl Walpurgis {
    /// Create a new game state.
    pub fn new(ctx: &mut Context, assets: &settings::Assets) -> WalpurgisResult<Self> {
        // Load/create resources here: images, fonts, sounds, etc.
        Ok(Walpurgis {
            screen: screens::Screen::Core(BattleData::load_first_arena_and_test_player(ctx, &assets.root)?),
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
        // Update code here...
        while ggez::timer::check_update_time(ctx, 60) {
            if keyboard::is_key_pressed(ctx, KeyCode::W) {
                log::info!("W");
            }
            if keyboard::is_key_pressed(ctx, KeyCode::A) {
                log::info!("A");
            }
            if keyboard::is_key_pressed(ctx, KeyCode::S) {
                log::info!("S");
            }
            if keyboard::is_key_pressed(ctx, KeyCode::D) {
                log::info!("D");
            }
            // TODO: useful work.
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
            KeyCode::Space => {
                if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift + CTRL (Space): Down");
                } else if mods.intersects(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift or CTRL (Space): Down");
                } else {
                    log::info!("Space: Down");
                }
            }
            KeyCode::Return => {
                if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift + CTRL (Return): Down");
                } else if mods.intersects(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift or CTRL (Return): Down");
                } else {
                    log::info!("Return: Down");
                }
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, mods: KeyMods) {
        match key {
            KeyCode::Space => {
                if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift + CTRL (Space): Release");
                } else if mods.intersects(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift or CTRL (Space): Release");
                } else {
                    log::info!("Space: Release");
                }
            }
            KeyCode::Return => {
                if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift + CTRL (Return): Release");
                } else if mods.intersects(KeyMods::SHIFT | KeyMods::CTRL) {
                    log::info!("Shift or CTRL (Return): Release");
                } else {
                    log::info!("Return: Release");
                }
            }
            _ => (),
        }
    }
}
