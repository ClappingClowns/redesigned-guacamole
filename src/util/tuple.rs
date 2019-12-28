pub fn transpose<T0, T1, S0, S1>(((t0, s0), (t1, s1)): ((T0, S0), (T1, S1))) -> ((T0, T1), (S0, S1)) {
    ((t0, t1), (s0, s1))
}

pub fn flip_tuple_vec<T0, T1>(vec: Vec<(T0, T1)>) -> Vec<(T1, T0)> {
    vec.into_iter().map(|(t0, t1)| (t1, t0)).collect()
}
