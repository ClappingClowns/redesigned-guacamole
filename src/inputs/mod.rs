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
#[derive(Default, Debug)]
pub struct InputScheme {
    // walk_left: Button,
    // walk_right: Button,
    // dash: Button,
    // jump: Button,
    // attack: Button,
    // shield: Button,
    // ability_buttons: Vec<Button>,
}
