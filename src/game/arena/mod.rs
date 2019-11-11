use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};
use ron::de::from_reader;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::path::Path;

use crate::util::result::WalpurgisResult;

mod platform;
use platform::*;

/// Stores data for the `Arena` outside of actual players.
#[derive(Debug, Serialize, Deserialize)]
pub struct Arena {
    /// Name of the Arena.
    name: String,
    /// `ggez`-specific. Not really used for anything atm.
    #[serde(skip)]
    mode: Option<BlendMode>,
    // background_images: Vec<ggez::Image>,
    // soundtracks: Vec<ggez::SoundData>,
    platforms: Vec<Platform>,
}

impl Arena {
    // TODO: remove this once we don't need it anymore
    /// Load the first arena in the arena directory.
    pub fn load_first<P: AsRef<Path>>(arena_dir: P) -> WalpurgisResult<Self> {
        let arena_dir = arena_dir.as_ref();
        log::info!("Loading first arena from assets directory: `{}`", arena_dir.display());

        // Really should be using the `glob` crate but don't want to
        // introduce an extra dependency just for this.
        let opt_arena_file = fs::read_dir(arena_dir)
            .and_then(|mut entries| entries.next().transpose())?;

        if let Some(arena_file) = opt_arena_file {
            Arena::load(arena_file.path())
        } else {
            Err(format!("No arena file found in the directory `{}`.", arena_dir.display()))?
        }
    }

    /// Tries to load an `Arena` from the given file.
    pub fn load<P: AsRef<Path>>(arena_file: P) -> WalpurgisResult<Self> {
        let f = File::open(arena_file)?;
        Ok(from_reader(f)?)
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
