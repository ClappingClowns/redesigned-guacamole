//! A few utilities for taking the cartesian product and square of iterators.

/// Computes the cartesian product of two iterators. Requires that the iterators iterate over
/// copyable elements and that the second iterator be clonable.
pub fn product<T, S, IIT, IS, IIS>(tt0: IIT, tt1: IIS) -> impl std::iter::Iterator<Item = (T, S)>
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
pub fn unique_square<T, IT, IIT>(tt: IIT) -> impl std::iter::Iterator<Item = (T, T)>
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

#[cfg(test)]
mod test {
    use super::*;

    const fn num_list1() -> [u32; 3] {
        [1, 2, 3]
    }
    const fn num_list2() -> [u32; 3] {
        [4, 5, 6]
    }

    const fn correct_product() -> [(u32,u32); 9] {
        [(1, 4), (1, 5), (1, 6),
         (2, 4), (2, 5), (2, 6),
         (3, 4), (3, 5), (3, 6),]
    }
    const fn correct_square() -> [(u32,u32); 3] {
        [(1, 2), (1, 3), (2, 3)]
    }

    fn pair_matches<E> (tp1: &(E,E), tp2: &(E,E)) -> bool
    where E: Eq + Copy
    {
        (tp1 == tp2) ||
        (tp1 == &(tp2.1, tp2.0))
    }

    fn pair_matches2<E> (tp1: (&E,&E), tp2: (&E,&E)) -> bool {
        (std::ptr::eq(tp1.0, tp2.0) && std::ptr::eq(tp1.1, tp2.1)) ||
        (std::ptr::eq(tp1.0, tp2.1) && std::ptr::eq(tp1.1, tp2.0))
    }

    #[test]
    fn cartesian_product_test() {
        let list1 = num_list1();
        let list2 = num_list2();
        let pairs: Vec<_> = product(&list1, &list2).map(|t| (*t.0, *t.1)).collect();
        assert!(pairs.len() == correct_product().len());
        for element in correct_product().iter() {
            assert!(pairs.iter().filter(|a| pair_matches(a, element)).count() == 1);
        }
    }

    #[test]
    fn cartesian_square_test() {
        let list = num_list1();
        let pairs: Vec<_> = unique_square(&list).map(|t| (*t.0, *t.1)).collect();
        assert!(pairs.len() == correct_square().len());
        for element in correct_square().iter() {
            assert!(pairs.iter().filter(|a| pair_matches(a, element)).count() == 1);
        }
    }
}
