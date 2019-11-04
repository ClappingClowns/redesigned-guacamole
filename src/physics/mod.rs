//! A collection of structs, traits, and functions for use with physics.
//!
//! ## Collisions
//! The game engine will check for collisions. It will pairwise compare all Collidable objects.
//!
//! ### Algorithmic complexity
//! If we have `n` `Collidable` objects, each with `b` `BoundingBoxes`, then we can iterate over
//! each collidable, iterate over every collidable, iterate over the pairs of bounding boxes in
//! each, and finally inflict collisions, resulting in runtime complexity `O(b*n^2)`.
//!
//! ### Parallelization
//! Will likely need to benchmark parallel and single-threaded versions of the code.
//!
//! We’ll deal with it when perf becomes an issue.

pub mod collision;
pub use collision::{Collidable, Effect, Collision};
pub mod obb;
pub use obb::BoundingBox;
