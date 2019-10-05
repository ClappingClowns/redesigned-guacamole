//! # Clapping Clowns - Design Document
//! ## Goal
//! Build a game similar to Super Smash Bros, Rivals of Aether, Street Fighter, etc,
//! except in Rust, with a skill tree and multiple races.
//! ## Game Engine
//! ### Ggez
//! * Very flexible & yet simple.
//! * Let’s you declare how often updates (i.e. the physics engine) should be run
//! * You can say: “I want the Physics Engine to run every 20 ms”. If there’s an OS glitch of some
//! sort & 40 ms has past since the last Physics Engine run, the game engine will call the Physics Engine twice.
//! * Targeted specifically for 2D games
//! * Access to GFX hal reference in case we want to side-step the game engine & render 3D objects
//!     * What is GFX hal?
//!     * It’s a Rust wrapper for graphics libraries on popular platforms like Linux, Windows and Mac.
//! This means you write graphics code once using GFX hal & it will take care of making sure your code
//! works with OpenGL/Vulkan (Linux), DirectX (Windows) and Metal (Mac).
//! * Can write shaders
//! * Game engine support for instancing (link2)
//! * Very simple audio & config management (engine can take care of zip files too)
//! 
//! Downsides:
//! * Ggez is not ECS-based. Should we encounter performance issues, we will likely have to parallelize the
//! game ourselves (which can be a good thing or a bad thing--good because better customizability, bad because more effort).
//! However, our game is relatively simple, so this should be a non-issue.
//! 
//! ## Data Types
//! ### Force
//! We have two kinds of force in this game:
//! One that inflicts damage
//! One that does not inflict damage
//! #### Benign Force
//! In Smash, when a character pushes another character, they get moved to the edge of the platform but no damage is inflicted.
//! In line with this, we will have a `Benign` force in our game that pushes objects without inflicting any damage on them.
//! A `Benign` force will simply contain a 2D vector that represents the force magnitude and direction.
//! #### Destructive Force
//! In Smash, when a character attacks another character, they get damage inflicted on them.
//! In line with this, we’ll have a `Dangerous` force. This will be used when players (and projectiles) attack one another.
//! `Dangerous` force will also contain a 2D vector representing the force magnitude and direction.
//! However, in addition, it will have an extra parameter, granting us some leeway (i.e. allowing using factors outside of standard physics to increase or decrease damage;
//! critical hits, for instance, would use this).
//! ```
//! enum Force {
//!     /// Doesn't hurt the entity the force is being applied to
//!     Benign(na::Vector2<f32>/* 2D vector representing the force magnitude & direction */)
//!     /// Hurts the entity the force is being applied to
//!     Dangerous(na::Vector2<f32>/* 2D vector representing the force magnitude & direction */, 
//!                f32 /* damage (not sure about the units) */)
//! }
//! ```
//! ### BoundingBox
//! Used to wrap an object that occupies space. Used by the Collidable trait below.
//! ```
//! struct BoundingBox{
//!     dimensions: na::Vector2<f32>;
//!     // The next two are local
//!     position: na::Vector2<f32>;
//!     orientation: f32; // Maybe f64
//! }
//! ```
//! ### Collidable
//! Any object that can be collided with should implement this trait.
//! When object A collides with object B, both A and B should affect one another.
//! ```
//! trait Collidable {
//!      fn get_position() -> &na::Vector2<f32>;
//!      fn get_hitboxes() -> &[BoundingBox];
//!      /// Inflict some force on the object you collided with.
//!      fn inflict(my_bounding_box_index: usize, other_bounding_box_index: usize, obj: &mut dyn Collidable);
//!      /// Allow the object you collided with to inflict force on you
//!      fn get_hit_by(my_bounding_box_index: usize, f: &Force);
//! }
//! ```
//! ### GameState
//! This is the global game state. We are likely going to want to have a couple of menu screens and a game screen. Thus, in Rust, it could look like so:
//! ```
//! enum GameState {
//!     /// Landing screen for transitioning into another page
//!     /// Buttons: Single Player, Multi Player, Options, Exit
//!     StartMenu(...),
//!     /// Screen for player selection when playing with an AI
//!     SinglePlayerPage(...),
//!     /// Screen to allow players to connect games over LAN
//!     /// and pick players.
//!     MultiPlayerPage(...),
//!     /// Control audio / video settings.
//!     OptionsPage(...),
//!     /// The actual game.
//!     GameScreen(BattleData),
//! }
//! ```
//! ### BattleData
//! This is the data specific to each battle. Every battle between Fighters will be played in an Arena. Thus, it can look something like so:
//! ```
//! /// BattleData holds all battle specific data.
//! /// It runs all battle specific logic, including determining when a game
//! /// ends.
//! struct BattleData {
//!     /// How long the battle has been going on for.
//!      timer: Timer;
//!      /// Where damage isn't taken.
//!      safe_zones: Vec<Box<dyn Safezone>>;
//!      /// Holds the state of the players in battle.
//!      /// If Krishna and David were playing this game,
//!      /// `players.len()` would equal 2.
//!      players: Vec<Player>;
//!      /// The Arena.
//!      arena: Arena;
//! }
//! ```
//! Note that BattleData will satisfy the ggez::Drawable trait (requires implementing a `draw` method),
//! meaning it will be drawable to screen. It’ll likely just draw the arena and every player to screen.
//! ### SafeZone
//! Regions inside of which damage isn’t taken.
//! ```
//! trait SafeZone {
//!     fn is_inside(na::Vector2<f32>);
//! }
//! ```
//! ### Arena
//! Will satisfy the ggez::Drawable trait.
//! ```
//! struct Arena {
//!     /// Assets
//!     background_images: Vec<ggez::Image>;
//!     soundtracks: Vec<ggez::SoundData>;/// 
//!     platforms: Vec<Platform>;
//! }
//! ```
//! ### Platform
//! Will satisfy the ggez::Drawable trait.
//! Platforms will initially all look the same (i.e. same color, height & width).
//! ```
//! /// A platform is a 0-slope rectangle that players can stand on.
//! struct Platform {
//!     /// Start of the platform.
//!     start_point: na::Vector2<f32>;
//!     /// End of the platform.
//!     end_point: na::Vector2<f32>;
//! }/// 
//! impl Collidable for Platform {...}
//! ```
//! ### Player
//! Most complicated data type in the game.
//! Have a position, velocity and acceleration.
//! Force of gravity is constantly acting on players, pulling them towards the ground.
//! When jumping, the player gets a large acceleration (and thus Force, since F = ma) upwards.
//! The force of gravity slowly counteracts this acceleration and eventually brings players onto the ground.
//! Force of gravity could be an object that implements the Inflictor trait.
//! Players can also launch attacks on other players, which entails applying a dangerous force on other players.
//! ```
//! type FrameNumber = usize;
//! impl Collidable for Player { ... };
//! struct Player {
//!     /// Assets:
//!     /// Sprites for different states & attacks.
//!     assets: Vec<Sprite>;
//!     /// Soundtrack for different movement.
//!     soundtracks: Vec<SoundData>;
//!     /// Position on the screen
//!     position: na::Vector2<f32>;
//!     velocity: na::Vector2<f32>;
//!     acceleration: na::Vector2<f32> 
//!     stance: (VerticalStance, HorizontalStance);
//!     /// Movement the player is currently part of
//!     movement: (Action, FrameNumber) 
//!     race: Race;
//!     stats: Stats;
//!     skills: Skills;
//! }
//! /// Whether the player is on the ground, crawling or in the air.
//! /// Used when determining whether to launch air or ground attacks.
//! /// Also used when determining which sprites to render.
//! enum VerticalStance {
//!     InAir {
//!         jumps_spent: i32,
//!         stance: AirStance,
//!     },
//!     OnGround(GroundStance),
//! }
//! enum AirStance {
//!     FastFalling,
//!     Falling,
//!     Upping,
//!     AttackAction(AttackAction),
//!     ..
//! }
//! enum GroundStance {
//!     Standing,
//!     Crawling,
//!     AttackAction(AttackAction),
//! }/// 
//! /// Whether the player is facing left or right.
//! /// HorizontalStance can't be inferred from velocity b/c
//! /// a player could be facing right and still be traveling left.
//! enum HorizontalStance {
//!     Left,
//!     Right,
//! } 
//! enum Action {
//!     /// No Movement
//!     Stationary,
//!     Walk,
//!     Dash,
//!     Jump,
//!     AttackAction(AttackAction),
//! }
//! enum AttackAction {
//!     DashAttack,
//!     Shielding,
//!     SideAttack,
//!     UpAttack,
//!     DownAttack,
//!     OffensiveSpecial,
//!     DefensiveSpecial,
//!     Wildcard1,
//!     Wildcard2,
//! }
//! ```
//! ## Collision Logic
//! ### Who checks for collisions
//! The game engine will check for collisions. It will pairwise compare all Collidable objects.
//! Should not be a performance issue since we will never have less than 10 collidable objects.
//! ### Algorithmic complexity
//! If we have n Collidable objects, each with b bounding boxes, then we can do the following:
//! ```
//! collidables.for_each(|source| => {
//!      collisions = collidables.map((c) => get_collision(c, source)).filter(Some);
//!      collisions.for_each(|(obj_collided_with, bbox_index_pairs)| => {
//!          bbox_index_pairs.for_each(|indx_pairs| {
//!             source.inflict(indx_pairs.0, indx_pairs.1, obj_collided_with);
//!          });
//!      })
//! })
//! ``` 
//! `O(b*n^2)`
//! ### Parallelization
//! Will likely need to benchmark parallel and single-threaded versions of the code.
//! Shelve for now.
//! We’ll deal with it when perf becomes an issue.
//! ## Input Management
//! Inputs only affect Players. Thus we need to figure out a mapping from device inputs to Player updates.
//! Every time we want to update a Player, we will send a message (an Event).
//! Mapping from inputs to events
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
//! ### Directions
//! Ott-san has decided that we will only support 8 directions: 4 cardinal and 4 diagonal.
//! ### Frequency with which we handle input
//! After talking to our technical advisor, Chittur-san, we need to handle all inputs every frame.
//! In other words, we cannot buffer inputs and handle them once every k frames, where k > 1.
//! ## Rendering Details
//! Overlapping Attacks
//! If Player A launches an attack and so does Player B, their attacks could overlap. If their attacks overlap, which attack appears on top?
//! ##Tick Update
//! 1. Collision detection
//! 1. Platform/Floor Collision
//! 1. Input management
//! 1. Update components
//!     * Players
//!     * Arena
//! 1. Re-render
//! ## Tasks / Milestones
//! * Future tasks in order of importance:
//! * Support saves
//! * Support skill trees
//! * Support local multiplayer
//! * Add audio
//! Check initial game idea doc for more features!

fn main() {
 println!("Hello, world!");
}