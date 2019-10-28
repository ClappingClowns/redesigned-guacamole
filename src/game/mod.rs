//! A collection of structs useful for drawing and managing the core gameplay loop.
//!
//! ## Rendering Details
//! Overlapping Attacks
//! If Player A launches an attack and so does Player B, their attacks could overlap. If their attacks overlap, which attack appears on top?
mod battledata;
pub use battledata::*;

mod arena;
pub use arena::*;

mod player;
pub use player::*;
