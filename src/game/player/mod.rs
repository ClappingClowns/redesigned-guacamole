use ggez::nalgebra as na;

use crate::{inputs, physics};

mod meta;
use self::meta::*;

mod stance;
use self::stance::*;

mod action;
use self::action::*;

/// The current frame being run. Allows for approximately four seconds of frames.
pub type FrameNumber = u8;

#[derive(Debug)]
pub struct Player {
    /// The sprites for animating the character.
    sprites: Vec</*Sprite*/()>,
    /// The sounds made by the character.
    sfx: Vec</*SoundData*/()>,

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

impl physics::Collidable for Player {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[physics::BoundingBox] {
        // TODO
        &[]
    }
    fn get_effects(&self) {}
}

