#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

extern crate average;

extern crate core;

use core::iter::Iterator;

use average::Min;

#[test]
fn trivial() {
    let mut m = Min::new();
    m.add(1.);
    m.add(2.);
    assert_eq!(m.min(), 1.);
    m.add(-1.);
    m.add(1.);
    assert_eq!(m.min(), -1.)
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 1..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let min_total: Min = sequence.iter().map(|x| *x).collect();
        assert_eq!(min_total.min(), 1.);
        let mut min_left: Min = left.iter().map(|x| *x).collect();
        assert_eq!(min_left.min(), 1.);
        let min_right: Min = right.iter().map(|x| *x).collect();
        assert_eq!(min_right.min(), sequence[mid]);
        min_left.merge(&min_right);
        assert_eq!(min_total.min(), min_left.min());
    }
}
