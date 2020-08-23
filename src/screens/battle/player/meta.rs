/// Categories of basic attacks.
#[derive(Debug)]
pub enum BasicClass {
    Air,
    Heavy,
    Light,
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

/// Buffs, aka effects with a timeout that affect stats.
#[derive(Debug)]
pub enum Buff {
    // TODO: ALL THE BUFFS (AND DEBUFFS WHICH ARE ALSO BUFFS AND THERE IS NO REASON FOR THIS TO BE
    // CAPS HOLY CRAP IT'S ANOTHER LINE)
}

/// A comprehensive summary of stats and perks taken in the basic skill tree.
#[derive(Default, Debug)]
pub struct Stats {
    // TODO: ???
}

/// Abilities are special active skills.
#[derive(Debug)]
pub enum Ability {
    // TODO: ALL THE ABILITIES
}
