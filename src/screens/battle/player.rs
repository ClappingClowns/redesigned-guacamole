use ggez::{Context, GameResult};
use ggez::graphics::{Image, Drawable, DrawParam, Rect, BlendMode};
use ggez::nalgebra as na;

use crate::inputs::{HandleInput, Input};
use crate::physics::*;
use crate::physics::collision::*;
use crate::util::result::WalpurgisResult;

pub mod inputs;
use self::inputs::{InputScheme};

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

    bboxes: Vec <BoundingBox>,

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
    inputs: InputScheme,

    /// Tracking data for platform fall-through.
    platforms_to_ignore: Vec<usize>,
    touched_platforms: Vec<usize>,
}

impl HandleInput for Player {
    fn handle_input(&mut self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) {
        let actions = self.inputs.get_possible_actions(ctx, fire_once_key_buffer);
        for action in actions {
            match action {
                Action::Walk(HorizontalStance::Left) => {
                    if let VerticalStance::OnGround(_) = self.stance.0 {
                        log::info!("Walking left");
                        self.stance.1 = HorizontalStance::Left;
                        self.position[0] -= 2_f32;
                    }
                },
                Action::Walk(HorizontalStance::Right) => {
                    if let VerticalStance::OnGround(_) = self.stance.0 {
                        log::info!("Walking right");
                        self.stance.1 = HorizontalStance::Right;
                        self.position[0] += 2_f32;
                    }
                },
                _ => (),
            }
        }
    }
}

#[derive(Clone)]
pub struct Changes {
    pub force: na::Vector2<f32>,
    pub contacted_platforms: Vec<usize>,
}

impl Default for Changes {
    fn default() -> Self {
        Changes {
            force: na::Vector2::new(0_f32, 0_f32),
            contacted_platforms: vec![],
        }
    }
}

impl Mergeable for Changes {
    fn merge(&self, other: &Self) -> Self {
        Changes {
            force: self.force + other.force,
            contacted_platforms: self.contacted_platforms.iter()
                .cloned()
                .chain(other.contacted_platforms.iter().cloned())
                .collect(),
        }
    }
}

impl Collidable for Player {
    type ChangeSet = Changes;

    fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox] {
        self.bboxes.as_ref()
    }
    fn apply_changeset(&mut self, Changes { mut force, contacted_platforms }: Self::ChangeSet) {
        log::trace!("Running changeset application on player.");

        log::info!("Moving at velocity: {:?}", self.velocity);
        self.update_for_platforms(contacted_platforms, &mut force);
        self.handle_push(force);
    }
    fn handle_phys_update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.reset_for_update();
    }
    fn get_offset(&self) -> na::Vector2<f32> {
        self.position.clone()
    }
}


impl Drawable for Player {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        for bbox in &self.bboxes {
            let mut box_param = param;
            box_param.color = ggez::graphics::Color::from_rgba(255, 0, 0, 130);
            box_param.dest.x += self.position[0];
            box_param.dest.y += self.position[1];
            bbox.draw(ctx, box_param)?;
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

impl Player {
    fn reset_for_update(&mut self) {
        self.acceleration = na::Vector2::zeros();
    }
    fn update_for_platforms(
        &mut self,
        platforms: Vec<usize>,
        f: &mut na::Vector2<f32>,
    ) {
        self.touched_platforms = platforms;
        let mut touching_new_platform = false;
        for touched in self.touched_platforms.iter() {
            if !self.platforms_to_ignore.contains(touched) {
                touching_new_platform = true;
                break;
            }
        }
        // If falling (aka velocity is downwards) and we hit a platform
        // we aren't falling through, we want to stop.
        if touching_new_platform && self.velocity[1] > 0. {
            // TODO Fix slight offsets.
            self.acceleration[1] = -self.velocity[1];
            f[1] = 0.;
        }
    }
    pub fn handle_push(&mut self, dir: na::Vector2<f32>) {
        self.acceleration += dir;
    }
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
        BoundingBox {
            mode: None,
            pos: na::Vector2::new(0_f32, 0_f32),
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

        position: na::Vector2::new(100_f32, 0_f32),
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
        inputs: InputScheme::default(),

        platforms_to_ignore: vec![],
        touched_platforms: vec![],
    })
}
