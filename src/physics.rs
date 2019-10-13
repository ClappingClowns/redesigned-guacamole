//! A collection of structs, traits, and functions for use with physics.
//!
//! ## Regarding Collision
//! The game engine will check for collisions. It will pairwise compare all Collidable objects.
//!
//! We use the following nifty to detect collision of two bounding boxes:
//! >>> Two axes aligned boxes (of any dimension) overlap if and only if the projections to all axes overlap
//! - [Source](https://stackoverflow.com/a/20925869/6421681)
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
//!
//! ## TODO
//! ### Testcases
//! * Every method under the BoundingBox impl

use ggez::nalgebra as na;

type Radians = f32;

/// Denotes an `area` is being occupied.
#[derive(Debug)]
pub struct BoundingBox {
    /// The pos (x, h) of the bounds. +x goes up and +y goes right.
    pub pos: na::Vector2<f32>,
    /// The size (w, h) of the bounds.
    pub size: na::Vector2<f32>,
    /// Orientation, i.e. radians to rotate the box in the counterclockwise directions.
    pub ori: Radians,
}
impl BoundingBox {
    /// Rotates a point counterclockwise.
    fn rotate(point: na::Vector2<f32>, ori: Radians) -> na::Vector2<f32> {
        na::Vector2::new(
            ori.cos() * point[0] - ori.sin() * point[1],
            ori.sin() * point[0] + ori.cos() * point[1],
        )
    }

    /// The 4 corners of an untransformed `BoundingBox`, as columns.
    /// 
    /// The first column corresponds to the bottom-left,
    /// 
    /// the second column corresponds to the top-left,
    /// 
    /// the third column corresponds to the bottom-right,
    /// 
    /// the fourth column corresponds to the top-right.
    /// 
    /// ```
    /// 2 ---------------- 4
    /// |                  |
    /// |                  |
    /// 1 ---------------- 3
    /// ```
    fn base_corners(&self) -> na::Matrix2x4<f32> {
        na::Matrix4x2::new(
            0f32,         0f32,
            0f32,         self.size[1],
            self.size[0], 0f32,
            self.size[0], self.size[1]
        ).transpose()
    }
    /// A rotation matrix for turning a 2D point counterclockwise `ori` radians.
    fn rot_matrix(&self) -> na::Matrix2<f32> {
        na::Matrix2::new(
            self.ori.cos(), -self.ori.sin(),
            self.ori.sin(),  self.ori.cos(),
        )
    }
    /// A function to return the four corners of the `BoundingBox` after applying the necessary
    /// transformations.
    fn corners(&self) -> na::Matrix2x4<f32> {
        let mut rotated_corners = self.rot_matrix() * self.base_corners();
        rotated_corners.column_iter_mut().map(|c| c + self.pos);
        rotated_corners
    }
    /// Returns the min and max x and y values when projected to the x and y axis arranged in the
    /// following way:
    ///
    /// | minimum_x_location, maximum_x_location |
    /// 
    /// | minimum_y_location, maximum_y_location |
    fn bounds(&self) -> na::Matrix2<f32> {
        let corners = self.corners();
        let deref_map = |row, init, folding_fn: fn(_, _) -> _| {
            corners.row(row).iter().map(|f| *f).fold(init, folding_fn)
        };
        na::Matrix2::new(
            deref_map(0, std::f32::INFINITY, f32::min), deref_map(0, std::f32::NEG_INFINITY, f32::max),
            deref_map(1, std::f32::INFINITY, f32::min), deref_map(1, std::f32::NEG_INFINITY, f32::max),
        )
    }

    /// Check if a collision can be detected from one of the two boxes.
    /// Check the module-level doc to understand our collision detection algorithm.
    ///
    /// A full collision check requires two of these with flipped parameters, so this is termed
    /// `half` a collision check.
    fn check_half_collision(&self, basis: &BoundingBox) -> bool {
        let rhs = self.normalized_wrt(basis);
        let lhs_bounds = basis.size;
        let rhs_bounds = rhs.bounds();
        // Bounds checking is rather complicated.
        //
        // There are fundamentally four cases, but all four can be collapsed to the following two.
        // The key insight is that the maximum of the minimum of both bounds must be less than the
        // minimum of the maximum of both bounds for there to be an overlap of two bounds.

        f32::max(rhs_bounds[(0, 0)], 0f32) <= f32::min(rhs_bounds[(0, 1)], lhs_bounds[0])
            && f32::max(rhs_bounds[(1, 0)], 0f32) <= f32::min(rhs_bounds[(1, 1)], lhs_bounds[1])
    }
    /// Checks if two `BoundingBox`es collide.
    /// Check the module-level doc to understand our collision detection algorithm.
    ///
    /// The underlying logic is that if any edge can show a separation between the two boxes, then
    /// the two boxes do not intersect.
    fn check_collision(lhs: &BoundingBox, rhs: &BoundingBox) -> bool {
        lhs.check_half_collision(rhs) && rhs.check_half_collision(lhs)
    }

    /// Normalize a box w.r.t. a basis.
    fn normalized_wrt(&self, basis: &Self) -> Self {
        Self {
            pos: Self::rotate(self.pos - basis.pos, self.ori - basis.ori),
            size: self.size,
            ori: self.ori - basis.ori,
        }
    }
}

/// Any object that can be collided with should implement this trait.
/// When object A collides with object B, both A and B should affect one another.
pub trait Collidable {
    /// Gets the list of hitboxes comprising the person.
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox];
    /// (Final interface TBD) Gets a set of effects to apply.
    fn get_effects(&self);
}

/// Returns the details of a collision.
///
/// Bound by lifetime to a single `tick` of the program.
pub struct Collision<'tick> {
    /// The pair of `Collidable` objects that collided.
    objs: (&'tick dyn Collidable, &'tick dyn Collidable),
    /// A list of `BoundingBox`es that were detected to overlap.
    ///
    /// The bounding boxes on the left belongs to the `Collidable` on the left and vice versa.
    overlapping_hitboxes: Vec<(&'tick BoundingBox, &'tick BoundingBox)>,
}
impl<'tick> From<((&'tick dyn Collidable, &'tick dyn Collidable), Vec<(&'tick BoundingBox, &'tick BoundingBox)>)> for Collision<'tick> {
    fn from((e_pair, hb_collisions): ((&'tick dyn Collidable, &'tick dyn Collidable), Vec<(&'tick BoundingBox, &'tick BoundingBox)>)) -> Self {
        Self {
            objs: e_pair,
            overlapping_hitboxes: hb_collisions,
        }
    }
}

/// Computes the cartesian product of two iterators. Requires that the iterators iterate over
/// copyable elements and that the second iterator be clonable.
fn cartesian_product<T, S, IIT, IS, IIS>(tt0: IIT, tt1: IIS) -> impl std::iter::Iterator<Item = (T, S)>
where
    T: Copy,
    IS: std::iter::Iterator<Item = S> + std::clone::Clone,
    IIT: std::iter::IntoIterator<Item = T>,
    IIS: std::iter::IntoIterator<Item = S, IntoIter = IS>,
{
    let it1 = tt1.into_iter();
    tt0.into_iter().flat_map(move |e0| {
        it1.clone().map(move |e1| (e0, e1))
    })
}
/// Computes the cartesian square of an iterator. Requires that the iterator iterate over copyable
/// elements and the iterator itself be cloneable.
fn unique_cartesian_square<T, IT, IIT>(tt: IIT) -> impl std::iter::Iterator<Item = (T, T)>
where
    T: Copy,
    IT: std::iter::Iterator<Item = T> + std::clone::Clone,
    IIT: std::iter::IntoIterator<Item = T, IntoIter = IT>,
{
    let mut it = tt.into_iter();
    std::iter::from_fn(move || {
        Some((it.next()?, it.clone()))
    })
        .flat_map(|(t0, remaining)| remaining.map(move |t1| (t0, t1)))
}
/// Transposes a 2-tuple of 2-tuples where the tuples are bucketed by type.
fn transpose<T, S>(((t0, s0), (t1, s1)): ((T, S), (T, S))) -> ((T, T), (S, S)) {
    ((t0, t1), (s0, s1))
}
/// Check for hit box collisions between two `IntoIterator`s of `BoundingBox`es.
fn check_for_hb_collisions<'a, I, II>((hb0, hb1): (II, II)) -> Vec<(&'a BoundingBox, &'a BoundingBox)>
where
    I: std::iter::Iterator<Item = &'a BoundingBox> + std::clone::Clone,
    II: std::iter::IntoIterator<Item = &'a BoundingBox, IntoIter = I>,
{
    cartesian_product(hb0, hb1)
        .filter(|(hb0, hb1)| BoundingBox::check_collision(hb0, hb1))
        .collect()
}
/// Check for collisions within a slice of [`Collidable`]s
pub fn check_for_collisions<'tick>(entities: &[&'tick dyn Collidable]) -> Vec<Collision<'tick>> {
    let entity_with_hitboxes: Vec<_> = entities
        .iter()
        .map(|e| (*e, e.get_hitboxes()))
        .collect();
    unique_cartesian_square(entity_with_hitboxes)
        .map(transpose)
        .map(|(e_pair, hb_pair)| (e_pair, check_for_hb_collisions(hb_pair)))
        .filter(|(_, hb_collisions): &(_, Vec<_>)| !hb_collisions.is_empty())
        .map(Collision::from)
        .collect()
}
