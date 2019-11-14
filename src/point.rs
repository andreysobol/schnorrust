extern crate num_bigint;
use num_bigint::{BigUint};

pub struct Point{
    x: BigUint,
    y: BigUint,
}

pub fn equal_ponts(p1: Point, p2: Point) -> bool {
    (p1.x == p2.x) && (p1.y == p2.y)
}

pub fn sum_ponts(p1: Point, p2: Point) -> Point {
    p1
}