use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode};
use ggez::nalgebra as na;
use ron::de::from_reader;
use serde::{Serialize, Deserialize};
use std::fs::{self, DirEntry, File};
use std::io;
use std::path::Path;

use crate::physics::BoundingBox;
use crate::util::string::stringify;

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
    /// Returns the first arena file in the arena directory.
    pub fn pick_first<P: AsRef<Path>>(arena_dir: P) -> Result<String, String> {
        // Really should be using the `glob` crate but don't want to
        // introduce an extra dependency just for this.
        let paths = fs::read_dir(arena_dir).map_err(stringify)?
            .collect::<Result<Vec<DirEntry>, io::Error>>()
            .map_err(stringify)?;

        match paths[0].path().to_str() {
            Some(path) => Ok(path.to_string()),
            None => Err(format!("No arena found in the arena directory."))
        }
    }

    /// Tries to construct a new `Arena` from the given file.
    pub fn new<P: AsRef<Path>>(arena_file: P) -> Result<Self, String> {
        let f = File::open(arena_file).map_err(stringify)?;
        from_reader(f).map_err(stringify)
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
