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
use ggez::ContextBuilder;
use ggez::conf::{WindowSetup, WindowMode};
use ggez::event;

mod inputs;
mod logging;
mod physics;
mod screens;
mod settings;
mod util;
mod walpurgis;

use walpurgis::Walpurgis;

fn main() {
    let settings = settings::load().expect("Failed to parse settings.");
    logging::setup(&settings.logging).expect("Failed to setup logging.");
    log::debug!("{:?}", settings);

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("Walpurgis", "clapping-clowns")
            .window_setup(WindowSetup {
                title: "Walpurgis".to_owned(),
                ..WindowSetup::default()
            })
            .window_mode(WindowMode {
                resizable: true,
                ..WindowMode::default()
            })
           .build()
           .unwrap();

     // Construct a game.
     let mut my_game = match Walpurgis::new(&mut ctx, &settings.assets) {
        Ok(game) => game,
        Err(reason) => {
            log::error!("Game construction failed: {:?}", reason);
            return
        },
    };

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

