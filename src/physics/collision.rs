use ggez::nalgebra as na;

use crate::physics::obb::BoundingBox;
use crate::util::cartesian::{
    product as cartesian_product,
    unique_square as unique_cartesian_square,
};
use crate::game::meta::Buff;

pub enum Effect {
    Push(ggez::nalgebra::Vector2<f32>),
    Damage(f32),
    Buff(Buff),
}

/// Any object that can be collided with should implement this trait.
/// When object A collides with object B, both A and B should affect one another.
pub trait Collidable {
    type ChangeSet;
    /// Gets the list of hitboxes comprising the person.
    ///
    /// TODO: Make this reflect a tree of collidables that we can narrow down in a broad and narrow
    /// phase.
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox];
    /// (Final interface TBD) Gets a set of effects to apply.
    fn get_effects(&self, bb: &BoundingBox) -> Vec<Effect>;
    // first hitbox in pair belongs to self
    fn handle_collision<'tick, T: Collidable> (
        &self,
        other: &'tick T,
        hitbox_pairs: &[(&'tick BoundingBox, &'tick BoundingBox)],
    ) -> Self::ChangeSet;
    fn handle_phys_update(&mut self);
    fn get_offset(&self) -> na::Vector2<f32>;
}

/// Returns the details of a collision.
///
/// Bound by lifetime to a single `tick` of the program.
pub struct Collision<'tick, T: Collidable, S: Collidable> {
    pub ids: (usize, usize),
    /// The pair of `Collidable` objects that collided.
    pub objs: (&'tick T, &'tick S),
    /// A list of `BoundingBox`es that were detected to overlap.
    ///
    /// The bounding boxes on the left belongs to the `Collidable` on the left and vice versa.
    pub overlapping_hitboxes: Vec<(&'tick BoundingBox, &'tick BoundingBox)>,
}
impl<'tick, T: Collidable, S: Collidable> From<(((usize, &'tick T), (usize, &'tick S)), Vec<(&'tick BoundingBox, &'tick BoundingBox)>)> for Collision<'tick, T, S> {
    fn from((((id0, e0), (id1, e1)), hb_collisions): (((usize, &'tick T), (usize, &'tick S)), Vec<(&'tick BoundingBox, &'tick BoundingBox)>)) -> Self {
        Self {
            ids: (id0, id1),
            objs: (e0, e1),
            overlapping_hitboxes: hb_collisions,
        }
    }
}

/// Transposes a 2-tuple of 2-tuples where the tuples are bucketed by type.
fn transpose<T0, T1, S0, S1>(((t0, s0), (t1, s1)): ((T0, S0), (T1, S1))) -> ((T0, T1), (S0, S1)) {
    ((t0, t1), (s0, s1))
}
/// Check for hit box collisions between two `IntoIterator`s of `BoundingBox`es.
///
/// Applies the provided offset to all of the hitboxes in either the first or the second iterator
/// depending on which iterator was requested to be offset.
fn check_for_hb_collisions<'a, I, II>(offset_second: bool, offset: na::Vector2<f32>, (hb0, hb1): (II, II)) -> Vec<(&'a BoundingBox, &'a BoundingBox)>
where
    I: std::iter::Iterator<Item = &'a BoundingBox> + std::clone::Clone,
    II: std::iter::IntoIterator<Item = &'a BoundingBox, IntoIter = I>,
{
    // Use the second set of hitboxes as the first when requested to do so.
    let (hb0, hb1) = if offset_second {
        (hb1, hb0)
    } else {
        (hb0, hb1)
    };
    let hb0_mapped_mem: Vec<_> = hb0.into_iter().map(|bb| (bb, BoundingBox {
        pos: bb.pos + offset,
        ..*bb
    })).collect();
    let hb0 = hb0_mapped_mem.iter();
    cartesian_product(hb0, hb1)
        .filter(|((_, offset_hb0), hb1)| {
            BoundingBox::check_collision(offset_hb0, hb1)
        })
        .map(|(&(hb0, _), hb1)| if offset_second { // flip again to counteract initial flip
            (hb1, hb0)
        } else {
            (hb0, hb1)
        })
        .collect()
}
/// Check for collisions within a slice of [`Collidable`]s
pub fn check_for_collisions<'tick, T:Collidable>(entities: &'tick[T]) -> Vec<Collision<'tick, T, T>> {
    let entity_with_hitboxes: Vec<_> = entities
        .iter()
        .enumerate()
        .map(|(id, e)| ((id, e), e.get_hitboxes()))
        .collect();
    unique_cartesian_square(entity_with_hitboxes)
        .map(transpose)
        .map(|(e_pair, hb_pair)| {
            // If the first list of hitboxes is shorter, we want to offset all the hitboxes of the
            // first vs the second.
            let should_offset_second = hb_pair.0.len() > hb_pair.1.len();
            let offset = if should_offset_second {
                (e_pair.1).1.get_offset() - (e_pair.0).1.get_offset()
            } else {
                (e_pair.0).1.get_offset() - (e_pair.1).1.get_offset()
            };
            (e_pair, check_for_hb_collisions(should_offset_second, offset, hb_pair))
        })
        .filter(|(_, hb_collisions): &(_, Vec<_>)| !hb_collisions.is_empty())
        .map(Collision::from)
        .collect()
}

/// Check for collisions between two slices of [`Collidable`]s
pub fn check_for_collision_pairs<'tick, T: Collidable, S: Collidable>(set1: &'tick[T], set2: &'tick[S]) 
    -> Vec<Collision<'tick, T, S>> {

    let set1_with_hitboxes: Vec<_> = set1
        .iter()
        .enumerate()
        .map(|(id, e)| ((id, e), e.get_hitboxes()))
        .collect();
    let set2_with_hitboxes: Vec<_> = set2
        .iter()
        .enumerate()
        .map(|(id, e)| ((id, e), e.get_hitboxes()))
        .collect();
    cartesian_product(set1_with_hitboxes, set2_with_hitboxes)
        .map(transpose)
        .map(|(e_pair, hb_pair)| {
            // If the first list of hitboxes is shorter, we want to offset all the hitboxes of the
            // first vs the second.
            let should_offset_second = hb_pair.0.len() > hb_pair.1.len();
            let offset = if should_offset_second {
                (e_pair.1).1.get_offset() - (e_pair.0).1.get_offset()
            } else {
                (e_pair.0).1.get_offset() - (e_pair.1).1.get_offset()
            };
            (e_pair, check_for_hb_collisions(should_offset_second, offset, hb_pair))
        })
        .filter(|(_, hb_collisions): &(_, Vec<_>)| !hb_collisions.is_empty())
        .map(Collision::from)
        .collect()
}



#[cfg(test)]
mod cartesian_collision_test {
    use super::*;
    type V2 = ggez::nalgebra::Vector2<f32>;

    pub struct DummyStruct {
        boxes: Vec<BoundingBox>
    }
    impl Collidable for DummyStruct {
        type ChangeSet = ();
        fn get_hitboxes<'tick>(&'tick self) -> &'tick[BoundingBox] {
            &self.boxes
        }
        fn get_effects(&self, bb: &BoundingBox) -> Vec<Effect> {
            vec![]
        }
        fn handle_collision<'tick, T: Collidable> (
            &self,
            other: &'tick T,
            hitbox_pairs: &[(&'tick BoundingBox, &'tick BoundingBox)],
        ) -> Self::ChangeSet { () }
        fn handle_phys_update(&mut self) {}
        fn get_offset(&self) -> na::Vector2<f32> {
            na::Vector2::new(0_f32, 0_f32)
        }
    }

    fn box_list1() -> Vec<BoundingBox> {
        vec![BoundingBox {
            mode: None,
            pos: V2::zeros(),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            mode: None,
            pos: V2::new(1.5, 0.),
            size: V2::new(1., 1.),
            ori: 0.,
        }]
    }
    fn box_list2() -> Vec<BoundingBox> {
        vec![BoundingBox {
            mode: None,
            pos: V2::new(-50.1, -50.1),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            mode: None,
            pos: V2::new(1.25, 0.),
            size: V2::new(1., 1.),
            ori: std::f32::consts::PI/4.,
        }]
    }
    fn box_list3() -> Vec<BoundingBox> {
        vec![BoundingBox {
            mode: None,
            pos: V2::new(50.1, 50.1),
            size: V2::new(1., 1.),
            ori: 0.,
        }, BoundingBox {
            mode: None,
            pos: V2::new(51.25, 50.),
            size: V2::new(1., 1.),
            ori: std::f32::consts::PI/4.,
        }]
    }

    fn pair_matches<E> (tp1: &(E,E), tp2: &(E,E)) -> bool
        where E: Eq
    {
        (tp1 == tp2) || (tp1.0 == tp2.1 && tp1.1 == tp2.0)
    }

    fn pair_matches2<E> (tp1: (&E,&E), tp2: (&E,&E)) -> bool {
        (std::ptr::eq(tp1.0, tp2.0) && std::ptr::eq(tp1.1, tp2.1)) ||
        (std::ptr::eq(tp1.0, tp2.1) && std::ptr::eq(tp1.1, tp2.0))
    }

    #[test]
    fn hb_collisions_test() {
        let boxes1 = box_list1();
        let boxes2 = box_list2();
        let correct_collisions = vec![(&boxes1[0], &boxes2[1]), (&boxes1[1], &boxes2[1])];
        let pairs = check_for_hb_collisions(false, na::Vector2::new(0_f32, 0_f32), (&boxes1, &boxes2));
        assert!(pairs.len() == correct_collisions.len());

        for element in correct_collisions.iter() {
            assert!(pairs.iter().filter(|a| pair_matches2(**a, *element)).count() == 1);
        }
    }

    #[test]
    fn flipped_hb_collisions_test() {
        let boxes1 = box_list1();
        let boxes2 = box_list2();
        let correct_collisions = vec![(&boxes1[0], &boxes2[1]), (&boxes1[1], &boxes2[1])];
        let pairs = check_for_hb_collisions(true, na::Vector2::new(0_f32, 0_f32), (&boxes1, &boxes2));
        assert!(pairs.len() == correct_collisions.len());

        for element in correct_collisions.iter() {
            assert!(pairs.iter().filter(|a| pair_matches2(**a, *element)).count() == 1);
        }
    }

    #[test]
    fn collisions_test() {
        let els: Vec<_> = [box_list1, box_list2, box_list3].iter()
            .map(|hb_fn| DummyStruct { boxes: hb_fn() })
            .collect();
        let el_refs: Vec<_> = els.iter().collect();
        let hb_refs: Vec<Vec<_>> = els.iter()
            .map(|e| e.get_hitboxes().iter().collect())
            .collect();

        let mut collisions = check_for_collisions(els.as_slice());
        assert!(collisions.len() == 1);

        let Collision {
            ids: (id0, id1),
            overlapping_hitboxes: overlaps,
            objs: (obj0, obj1),
        } = collisions.pop().unwrap();

        if std::ptr::eq(obj0, el_refs[0]) {
            assert!(std::ptr::eq(obj0, el_refs[0]));
            assert!(std::ptr::eq(obj1, el_refs[1]));
        } else {
            assert!(std::ptr::eq(obj1, el_refs[0]));
            assert!(std::ptr::eq(obj0, el_refs[1]));
        }

        assert!(overlaps.len() == 2);
        let match0 = (hb_refs[0][0], hb_refs[1][1]);
        let match1 = (hb_refs[0][1], hb_refs[1][1]);
        if pair_matches2(overlaps[0], match0) {
            assert!(pair_matches2(overlaps[0], match0));
            assert!(pair_matches2(overlaps[1], match1));
        } else {
            assert!(pair_matches2(overlaps[1], match0));
            assert!(pair_matches2(overlaps[0], match1));
        }
    }
}
