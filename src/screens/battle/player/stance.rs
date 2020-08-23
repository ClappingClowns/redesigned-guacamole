use super::action::Attack;

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
