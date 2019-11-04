use ggez::nalgebra as na;
use serde::{Deserialize};

use crate::physics::{Collidable, Effect, Collision};

type Radians = f32;

/// Denotes an `area` is being occupied.
#[derive(Debug, Clone, Deserialize)]
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
        const ORI_EPSILON: f32 = 1e-7;

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
    pub fn check_collision(lhs: &BoundingBox, rhs: &BoundingBox) -> bool {
        // If bounding boxes have almost the same EPSILON, only half of the check is necessary
        // since it approximates an AABB check. The secondary check becomes a simple offset of the
        // first check.
        let does_not_need_secondary_check = BoundingBox::is_almost_axis_aligned(lhs, rhs);

        lhs.check_half_collision(rhs) && (does_not_need_secondary_check || rhs.check_half_collision(lhs))
    }

    /// Checks if two bounding boxes are almost axis aligned. Namely, if their orientations are
    /// almost equivalent.
    fn is_almost_axis_aligned(lhs: &BoundingBox, rhs: &BoundingBox) -> bool {
        const ORI_EPSILON: f32 = 1e-7;
        (rhs.ori - lhs.ori).abs() < ORI_EPSILON
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
impl Collidable for BoundingBox {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox] {
        std::slice::from_ref(self)
    }
    /// (Final interface TBD) Gets a set of effects to apply.
    fn get_effects(&self, bb: &BoundingBox) -> Vec<Effect> {
        vec![]
    }
    fn handle_collision(&self, collision: &Collision) {}
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
