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

use ggez::nalgebra as na;

type Radians = f32;

/// Denotes an `area` is being occupied.
#[derive(Debug, Clone)]
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

    /// The 4 corners of an untransformed `BoundingBox`, as columns. The diagram indicates which
    /// column corresponds to which corner when the bounding box is not rotated.
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
        for mut c in rotated_corners.column_iter_mut() {
            c += &self.pos;
        } // to consume the map
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
    /// A full collision check requires two calls to this with flipped parameters, so this is
    /// termed `half` a collision check.
    fn check_half_collision(&self, basis: &BoundingBox) -> bool {
        let rhs = self.normalized_wrt(basis);
        let lhs_bounds = basis.size;
        let rhs_bounds = rhs.bounds();
        // Bounds checking is rather ... complicated/involved.
        //
        // There are four obvious cases, but all four can be collapsed to the following two.
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
/// elements and the iterator itself be cloneable. Only one of non-unique elements are emitted and
/// elements whose members are identical are eliminated.
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

#[cfg(test)]
mod obb_test {
    use super::*;

    type V2 = na::Vector2<f32>;
    fn approx_eq(a: V2, b: V2) -> bool {
        const EPSILON: f32 = 1e-5;
        (a[0] - b[0]).abs() < EPSILON && (a[1] - b[1]).abs() < EPSILON
    }
    fn build_bounding() -> BoundingBox {
        BoundingBox {
            pos: V2::new(1., 2.),
            size: V2::new(3., 4.),
            ori: std::f32::consts::PI / 2.,
        }
    }

    #[test]
    fn bounding_box_rotate() {
        let ex = V2::new(0.5, 0.5);
        let ex_quarter = BoundingBox::rotate(ex, std::f32::consts::PI / 2.);
        assert!(approx_eq(ex, V2::new(0.5, 0.5)));
        assert!(approx_eq(ex_quarter, V2::new(-0.5, 0.5)));
        let ex_half = BoundingBox::rotate(ex_quarter, std::f32::consts::PI / 2.);
        assert!(approx_eq(ex_half, V2::new(-0.5, -0.5)));
    }

    #[test]
    fn obb_base_corners() {
        let corners = build_bounding().base_corners();
        assert!(approx_eq(V2::from(corners.column(0)), V2::new(0., 0.)));
        assert!(approx_eq(V2::from(corners.column(1)), V2::new(0., 4.)));
        assert!(approx_eq(V2::from(corners.column(2)), V2::new(3., 0.)));
        assert!(approx_eq(V2::from(corners.column(3)), V2::new(3., 4.)));
    }

    #[test]
    fn obb_rot_matrix() {
        let rot = build_bounding().rot_matrix();
        let ex = V2::new(0.5, 0.5);
        assert!(approx_eq(rot * ex, V2::new(-0.5, 0.5)));
    }

    #[test]
    fn obb_corners() {
        let corners = build_bounding().corners();
        assert!(approx_eq(V2::from(corners.column(0)), V2::new( 1., 2.)));
        assert!(approx_eq(V2::from(corners.column(1)), V2::new(-3., 2.)));
        assert!(approx_eq(V2::from(corners.column(2)), V2::new( 1., 5.)));
        assert!(approx_eq(V2::from(corners.column(3)), V2::new(-3., 5.)));
    }

    #[test]
    fn obb_bounds() {
        let bounds = build_bounding().bounds();
        assert!(approx_eq(V2::from(bounds.row(0).transpose()), V2::new(-3., 1.)));
        assert!(approx_eq(V2::from(bounds.row(1).transpose()), V2::new( 2., 5.)));
    }

    fn colliding_boxes() -> (BoundingBox, BoundingBox) {
        (BoundingBox {
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        })
    }
    fn separate_boxes() -> (BoundingBox, BoundingBox)  {
        (BoundingBox {
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::new(-0.1, -0.1),
            size: V2::new(1., 1.),
            ori: std::f32::consts::PI,
        })
    }
    fn pathological_separate_boxes() -> (BoundingBox, BoundingBox) {
        (BoundingBox {
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::new(1.5, 0.5),
            size: V2::new(5., 0.5),
            ori: std::f32::consts::PI / 4.,
        })
    }

    #[test]
    fn obb_half_collision() {
        { // separate
            let (a, b) = separate_boxes();
            assert!(!a.check_half_collision(&b))
        }
        { // colliding
            let (a, b) = colliding_boxes();
            assert!(a.check_half_collision(&b));
        }
        { // pathological separate
            let (a, b) = pathological_separate_boxes();
            assert!(a.check_half_collision(&b));
        }
    }

    #[test]
    fn obb_collision() {
        { // separate
            let (a, b) = separate_boxes();
            assert!(!BoundingBox::check_collision(&a, &b));
        }
        { // colliding
            let (a, b) = colliding_boxes();
            assert!(BoundingBox::check_collision(&a, &b));
        }
        { // pathological separate
            let (a, b) = pathological_separate_boxes();
            assert!(!BoundingBox::check_collision(&a, &b));
        }
    }

    #[test]
    fn obb_norm_wrt() {
        let b = build_bounding();
        let normed = b.normalized_wrt(&b);
        assert!(approx_eq(normed.pos, V2::new(0., 0.)));
        assert!(approx_eq(normed.size, V2::new(3., 4.)));
        assert!(normed.ori.abs() < 1e-5);
        let mut weird_b = b.clone();
        weird_b.pos[0] = 0.;
        weird_b.ori = -std::f32::consts::PI / 2.;
        let normed = b.normalized_wrt(&weird_b);
        assert!(approx_eq(normed.pos, V2::new(-1., 0.)));
        assert!(approx_eq(normed.size, V2::new(3., 4.)));
        assert!((normed.ori - std::f32::consts::PI).abs() < 1e-5);
    }
}

#[cfg(test)]
mod cartesian_test {
    use super::*;

    pub struct DummyStruct {
        boxes: Vec<BoundingBox>
    }

    impl Collidable for DummyStruct {
        fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox] {
            &self.boxes
        }
        fn get_effects(&self) {}
    }

    type V2 = na::Vector2<f32>;
    fn num_list1() -> [u32; 3] {
        [1, 2, 3]
    }

    fn num_list2() -> [u32; 3] {
        [4, 5, 6]
    }

    fn correct_product() -> [(u32,u32); 9] {
        [(1, 4), (1, 5), (1, 6),
         (2, 4), (2, 5), (2, 6),
         (3, 4), (3, 5), (3, 6),]
    }

    fn correct_square() -> [(u32,u32); 3] {
        [(1, 2), (1, 3), (2, 3)]
    }

    fn box_list1() -> Vec<BoundingBox> {
        vec![BoundingBox {
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::new(1.5, 0.),
            size: V2::new(1., 1.),
            ori: 0.,
        }]
    }

    fn box_list2() -> Vec<BoundingBox> {
        vec![BoundingBox {
            pos: V2::new(-50.1, -50.1),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::new(1.25, 0.),
            size: V2::new(1., 1.),
            ori: std::f32::consts::PI/4.,
        }]
    }

    fn box_list3() -> Vec<BoundingBox> {
        vec![BoundingBox {
            pos: V2::new(50.1, 50.1),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            pos: V2::new(51.25, 50.),
            size: V2::new(1., 1.),
            ori: std::f32::consts::PI/4.,
        }]
    }

    fn pair_matches<E> (tp1: &(E,E), tp2: &(E,E)) -> bool 
    where E: Eq + Copy
    {
        (tp1 == tp2) ||
        (tp1 == &(tp2.1, tp2.0))
    }

    fn pair_matches2<E> (tp1: &(&E,&E), tp2: &(&E,&E)) -> bool {
        (std::ptr::eq(tp1.0, tp2.0) && std::ptr::eq(tp1.1, tp2.1)) ||
        (std::ptr::eq(tp1.0, tp2.1) && std::ptr::eq(tp1.1, tp2.0))
    }


    #[test]
    fn cartesian_product_test() {
        let list1 = num_list1();
        let list2 = num_list2();
        let pairs: Vec<_> = cartesian_product(&list1, &list2).map(|t| (*t.0, *t.1)).collect();
        assert!(pairs.len() == correct_product().len());
        for element in correct_product().iter() {
            assert!(pairs.iter().filter(|a| pair_matches(a, element)).count() == 1);
        }
        
    }

    #[test]
    fn cartesian_square_test() {
        let list = num_list1();
        let pairs: Vec<_> = unique_cartesian_square(&list).map(|t| (*t.0, *t.1)).collect();
        assert!(pairs.len() == correct_square().len());
        for element in correct_square().iter() {
            assert!(pairs.iter().filter(|a| pair_matches(a, element)).count() == 1);
        }
        
    }

    #[test]
    fn hb_collisions_test() {
        let boxes1 = box_list1();
        let boxes2 = box_list2();
        let correct_collisions = vec![(&boxes1[0], &boxes2[1]), (&boxes1[1], &boxes2[1])];
        let pairs = check_for_hb_collisions((&boxes1, &boxes2));
        assert!(pairs.len() == correct_collisions.len());

        for element in correct_collisions.iter() {
            assert!(pairs.iter().filter(|a| pair_matches2(a, element)).count() == 1);
        }
    }

    #[test]
    fn collisions_test() {
        let boxes1 = box_list1();
        let boxes2 = box_list2();
        let boxes3 = box_list3();
        let element1: &dyn Collidable = &DummyStruct {
            boxes: boxes1
        };
        let element2: &dyn Collidable = &DummyStruct {
            boxes: boxes2
        };
        let element3: &dyn Collidable = &DummyStruct {
            boxes: boxes3
        };
        let elements = vec![element1, element2, element3];
        let collisions = check_for_collisions(&elements[..]);
        assert!(collisions.len() == 1);
        if std::ptr::eq(collisions[0].objs.0, element1) {
            assert!(std::ptr::eq(collisions[0].objs.0, element1));
            assert!(std::ptr::eq(collisions[0].objs.1, element2));
        } else {
            assert!(std::ptr::eq(collisions[0].objs.1, element1));
            assert!(std::ptr::eq(collisions[0].objs.0, element2));
        }
        assert!(collisions[0].overlapping_hitboxes.len() == 2);
        if pair_matches2(&collisions[0].overlapping_hitboxes[0], &(&element1.get_hitboxes()[0], &element2.get_hitboxes()[1])) {
            assert!(pair_matches2(&collisions[0].overlapping_hitboxes[0], &(&element1.get_hitboxes()[0], &element2.get_hitboxes()[1])));
            assert!(pair_matches2(&collisions[0].overlapping_hitboxes[1], &(&element1.get_hitboxes()[1], &element2.get_hitboxes()[1])));
        } else {
            assert!(pair_matches2(&collisions[0].overlapping_hitboxes[1], &(&element1.get_hitboxes()[0], &element2.get_hitboxes()[1])));
            assert!(pair_matches2(&collisions[0].overlapping_hitboxes[0], &(&element1.get_hitboxes()[1], &element2.get_hitboxes()[1])));
        }
    }
}