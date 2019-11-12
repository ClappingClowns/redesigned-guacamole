//! Inputs only affect Players. Thus we need to figure out a mapping from device inputs to Player updates.
//! Every time we want to update a Player, we will send a message (an Event).
//! Mapping from inputs to events in the basic scheme:
//! 
//! |   Input                  |   Event                          |
//! |--------------------------|----------------------------------|
//! | A / D                    |  (Walk, Left \| Right)           |
//! | Shift                    | Dash                             |
//! | Space                    |  Jump                            |
//! | Mouse 0                  | Attack                           |
//! | Mouse 1                  | Heavy                            |
//! | Attack while dashing     | DashAttack                       |
//! | Q                        | Shielding                        |
//! | W + Attack               | UpAttack                         |
//! | S + Attack               | DownAttack                       |
//! | Configurable (1)         | OffensiveSpecial                 |
//! | Configurable (2)         | DefensiveSpeical                 |
//! | Configurable (3)         | Wildcard1                        |
//! | Configurable (4)         | Wildcard2                        |
//! | Configurable (5)         | Wildcard3                        |
//!
//! ### Directions
//! Ott-san has decided that we will only support 8 directions: 4 cardinal and 4 diagonal.
//!
//! ### Frequency with which we handle input
//! After talking to our technical advisor, Chittur-san, we need to handle all inputs every frame.
//! In other words, we cannot buffer inputs and handle them once every k frames, where k > 1.

/// # Note
/// Input management will look sth like so:
/// ```
///                                  _________________
///                                 |                 | <--------------- Keyboard Input Stream
/// Event Stream <----------------- | Stateful Mapper | <--------------- Mouse Input Stream
///                                 |_________________| <--------------- Joystick Input Stream
/// ```
/// 
use ggez::Context;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;

use super::action::Action;
use super::stance::HorizontalStance;

use crate::inputs::Input;

#[derive(Debug)]
pub struct InputScheme {
    continuous: ContinuousScheme,
    fire_once: FireOnceScheme,
    // dash: Button,
    // jump: Button,
    // attack: Button,
    // shield: Button,
    // ability_buttons: Vec<Button>,
}

impl InputScheme {
    pub fn get_possible_actions(&self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) -> Vec<Action> {
        let mut all_actions = self.continuous.get_possible_actions(ctx);
        all_actions.append(&mut self.fire_once.get_possible_actions(ctx, fire_once_key_buffer));
        all_actions
    }
}

#[derive(Debug)]
pub struct ContinuousScheme {
    pub walk_left: (KeyCode, KeyMods),
    pub walk_right: (KeyCode, KeyMods),
}

impl ContinuousScheme {
    pub fn get_possible_actions(&self, ctx: &mut Context) -> Vec<Action> {
        let mut actions = vec![];
        let mods = keyboard::active_mods(ctx);
        for key in keyboard::pressed_keys(ctx) {
            if (*key, mods) == self.walk_left {
                actions.push(Action::Walk(HorizontalStance::Left));
            }
            if (*key, mods) == self.walk_right {
                actions.push(Action::Walk(HorizontalStance::Right));
            }
        }
        actions
    }
}

#[derive(Debug)]
pub struct FireOnceScheme;

impl FireOnceScheme {
    pub fn get_possible_actions(&self, ctx: &mut Context, fire_once_key_buffer: &Vec<Input>) -> Vec<Action> {
        vec![]
    }
}

impl Default for InputScheme {
    fn default() -> Self {
        InputScheme {
            continuous: ContinuousScheme {
                walk_left: (KeyCode::A, KeyMods::NONE),
                walk_right: (KeyCode::D, KeyMods::NONE),
            },
            fire_once: FireOnceScheme,
        }
    }
}
