use ggez::{Context, GameResult};
use ggez::graphics::{Image, Drawable, DrawParam, Rect, BlendMode};
use ggez::nalgebra as na;

use crate::{inputs, physics, util::result::WalpurgisResult};

pub mod meta;
use self::meta::*;

mod stance;
use self::stance::*;

mod action;
use self::action::*;

/// The current frame being run. Allows for approximately four seconds of frames.
pub type FrameNumber = u8;

#[derive(Debug)]
pub struct Player {
    /// `ggez`-specific. Not really used for anything atm.
    mode: Option<BlendMode>,

    /// The sprites for animating the character.
    sprites: Vec<Image>,
    /// The sounds made by the character.
    sfx: Vec</*SoundData*/()>,

    bboxes: Vec<physics::BoundingBox>,

    /// The position of the character.
    position: na::Vector2<f32>,
    /// The velocity of the character.
    velocity: na::Vector2<f32>,
    /// The acceleration of the character.
    acceleration: na::Vector2<f32>,

    /// Buffs currently in effect.
    buff: Vec<Buff>,

    /// Animation variations.
    stance: (VerticalStance, HorizontalStance),
    /// Animation state.
    movement: (Action, FrameNumber),

    /// The race of the player character.
    race: Race,
    /// Various stats.
    stats: Stats,
    /// The selected `Ability`s of the player character.
    abilities: Vec<Ability>,
    /// The input options allowed for a player.
    inputs: inputs::InputScheme,
}

/// A `Player` to be used for testing.
pub fn test_player(ctx: &mut Context) -> WalpurgisResult<Player> {
    let torso = Image::from_rgba8(
        ctx, 1 /* width */, 2 /* height */,
        &[
            255, 0, 0, 0,
            0, 255, 0, 0,
        ]
    )?;
    let bboxes = vec![
        physics::BoundingBox {
            mode: None,
            pos: na::Vector2::new(100_f32, 470_f32),
            size: na::Vector2::new(30_f32, 30_f32),
            ori: 0_f32,
        },
    ];

    Ok(Player {
        mode: None,
        sprites: vec![
            torso,
        ],
        sfx: vec![],

        position: na::Vector2::new(5_f32, 5_f32),
        velocity: na::Vector2::new(0_f32, 0_f32),
        acceleration: na::Vector2::new(0_f32, 0_f32),
        bboxes,

        buff: vec![],
        stance: (
            VerticalStance::OnGround(GroundStance::Standing),
            HorizontalStance::Left,
        ),
        movement: (Action::Idle, 0),

        race: Race::Alien,
        stats: Stats {},
        abilities: vec![],
        inputs: inputs::InputScheme {},
    })
}

impl physics::Collidable for Player {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[physics::BoundingBox] {
        // TODO
        &[]
    }
    fn get_effects(&self, bb: &physics::BoundingBox) -> Vec<physics::Effect> {
        vec![]
    }
    fn handle_collision(&self, collision: &physics::Collision) {}
}


impl Drawable for Player {
    fn draw(&self, ctx: &mut Context, mut param: DrawParam) -> GameResult {
        param.color = ggez::graphics::Color::from_rgba(255, 0, 0, 130);
        for bbox in &self.bboxes {
            bbox.draw(ctx, param)?;
        }
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
