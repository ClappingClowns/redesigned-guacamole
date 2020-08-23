// use std::any::TypeId; // Related to commented code.
use crate::{
    screens::battle::{
        platform::Platform,
        player::Player,
    },
    physics::{Collision, Collidable},
};

// Replace handle_x_x_collision with specialization once available.
type Changes<A: Collidable, B: Collidable> = (Option<<A as Collidable>::ChangeSet>, Option<<B as Collidable>::ChangeSet>);

/*
pub fn handle_collision<'tick, A: Collidable, B: Collidable>(
    c: Collision<'tick, A, B>
) -> Changes<A, B> {
    let platform = TypeId::of::<Platform>();
    let player = TypeId::of::<Player>();
    match (c.objs.0.type_id(), c.objs.1.type_id()) {
        (player, platform) => handle_platform_player_collision(c),
        (platform, player) => {
            let (b_changeset, a_changeset) = handle_platform_player_collision(c.flipped());
            Some((a_changeset, b_changeset))
        },
        (player, player) => handle_player_player_collision(c),
        (platform, platform) => handle_platform_platform_collision(c),
        _ => return (None, None),
    }
}
*/

pub fn handle_platform_platform_collision<'tick>(
    c: Collision<'tick, Platform, Platform>,
) -> Changes<Platform, Platform> {
    (None, None)
}
pub fn handle_player_player_collision<'tick>(
    c: Collision<'tick, Player, Player>,
) -> Changes<Player, Player> {
    (None, None)
}
pub fn handle_player_platform_collision<'tick>(
    c: Collision<'tick, Player, Platform>,
) -> Changes<Player, Platform> {
    (None, None)
}

