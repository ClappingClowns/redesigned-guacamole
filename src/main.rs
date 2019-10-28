//! # Goal
//! Build a game similar to Super Smash Bros, Rivals of Aether, Street Fighter, etc,
//! except in Rust, with a skill tree and multiple races.
//! ## Game Engine
//! ### Ggez
//! * Very flexible & yet simple.
//! * Let’s you declare how often updates (i.e. the physics engine) should be run
//! * You can say: “I want the Physics Engine to run every 20 ms”. If there’s an OS glitch of some
//! sort & 40 ms has past since the last Physics Engine run, the game engine will call the Physics Engine twice.
//! * Targeted specifically for 2D games
//! * Access to GFX hal reference in case we want to side-step the game engine & render 3D objects
//!     * What is GFX hal?
//!     * It’s a Rust wrapper for graphics libraries on popular platforms like Linux, Windows and Mac.
//! This means you write graphics code once using GFX hal & it will take care of making sure your code
//! works with OpenGL/Vulkan (Linux), DirectX (Windows) and Metal (Mac).
//! * Can write shaders
//! * Game engine support for instancing (link2)
//! * Very simple audio & config management (engine can take care of zip files too)
//! 
//! Downsides:
//! * Ggez is not ECS-based. Should we encounter performance issues, we will likely have to parallelize the
//! game ourselves (which can be a good thing or a bad thing--good because better customizability, bad because more effort).
//! However, our game is relatively simple, so this should be a non-issue.
//!
//! # Tasks / Milestones
//! ## Future tasks in order of importance:
//! * Support saves
//! * Support skill trees
//! * Support local multiplayer
//! * Add audio
//! Check initial game idea doc for more features!
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable, DrawParam};

mod screens;
mod physics;
mod inputs;
mod game;
mod logging;

fn main() {
    logging::setup().unwrap();

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("wip-redesigned-guac", "clapping-clowns")
           .build()
           .unwrap();

    let mut my_game = WIPRG::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

/// This is the global game state. We are likely going to want to have a couple of menu screens and a game screen. Thus, in Rust, it could look like so:
struct WIPRG {
    // TODO: Some shared state
    /// Screen specific state.
    screen: screens::Screen,
}
impl WIPRG {
    /// Create a new game state, referencing the provided `ggez` `Context`.
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources here: images, fonts, sounds, etc.
        Self {
            screen: screens::Screen::default(),
        }
    }
}

impl EventHandler for WIPRG {
    /// This executes a tick update.
    /// 1. Collision detection
    /// 2. Platform/Floor Collision
    /// 3. Input management
    /// 4. Update components
    ///     * Players
    ///     * Arena
    /// 5. Re-render
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        while ggez::timer::check_update_time(ctx, 60) {
            // TODO: useful work.
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context)-> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        match self.screen.draw(ctx, DrawParam::new()) {
            Ok(()) => (),
            Err(reason) => log::error!("{}", reason),
        };
        graphics::present(ctx)
    }
}
