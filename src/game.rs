//! A collection of structs useful for drawing and managing the core gameplay loop.
//!
//! ## Rendering Details
//! Overlapping Attacks
//! If Player A launches an attack and so does Player B, their attacks could overlap. If their attacks overlap, which attack appears on top?

use ggez::nalgebra as na;

use crate::{physics, inputs};

/// Whether the player character faces left or right.
#[derive(Debug)]
pub enum HorizontalStance {
    Left,
    Right,
}
/// What actions are currently being animated. As well as a bit of state.
#[derive(Debug)]
pub enum VerticalStance {
    InAir {
        jumps_spent: u32,
        stance: AirStance,
    },
    OnGround(GroundStance)
}
/// The animation state and counters while in the air.
#[derive(Debug)]
pub enum AirStance {
    FastFalling,
    Falling,
    Upping,
    Attack(Attack),
}
/// The animation state and counters while on the ground.
#[derive(Debug)]
pub enum GroundStance {
    Standing,
    Attack(Attack),
}
/// Actions available for the player to take.
#[derive(Debug)]
pub enum Action {
    Idle,
    Walk,
    Dash,
    Jump,
    Attack(Attack),
}
/// Different types of attacks.
#[derive(Debug)]
pub enum Attack {
    DashAttack,
    Shielding,
    Basics(BasicClass, AttackDir),
    Ability(Ability),
}
/// Categories of basic attacks.
#[derive(Debug)]
pub enum BasicClass {
    Air,
    Heavy,
    Light,
}
/// The direction of an attack.
#[derive(Debug)]
pub enum AttackDir {
    Up,
    Down,
    Side,
}
/// Abilities are special active skills.
#[derive(Debug)]
pub enum Ability {
    // TODO: ALL THE ABILITIES
}
/// Buffs, aka effects with a timeout that affect stats.
#[derive(Debug)]
pub enum Buff {
    // TODO: ALL THE BUFFS (AND DEBUFFS WHICH ARE ALSO BUFFS AND THERE IS NO REASON FOR THIS TO BE
    // CAPS HOLY CRAP IT'S ANOTHER LINE)
}
/// The race of the player character.
#[derive(Debug)]
pub enum Race {
    /// The aliens are the ultimate forms of biological evolution.
    Alien,
    /// The robots are the ultimate forms of technological evolution.
    Robot,
    /// An interesting energy-based race.
    Mage,
}
/// A comprehensive summary of stats and perks taken in the basic skill tree.
#[derive(Default, Debug)]
pub struct Stats {
    // TODO: ???
}
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

/// Denotes a static section of the `Arena`. Implements `ggez::Drawable`.
#[derive(Debug)]
pub struct Platform {
    /// The portion occupied by the platform.
    body: [physics::BoundingBox; 1],
    /// If a player is allowed to move through the platform.
    can_move_through: bool,
    // TODO: Add storage for the assets' handles.
}
impl physics::Collidable for Platform {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[physics::BoundingBox] {
        &self.body
    }
    fn get_effects(&self) {}
}

/// Stores data for the `Arena` outside of actual players. Will satisfy the `ggez::Drawable` trait.
#[derive(Debug)]
pub struct Arena {
     // background_images: Vec<ggez::Image>,
     // soundtracks: Vec<ggez::SoundData>,
     platforms: Vec<Platform>,
}
