extern crate num_bigint;
use num_bigint::{BigUint};

pub enum Point{
    Infinity,
    ExistingPoint{
        x: BigUint,
        y: BigUint,
    }
}

pub fn equal_points(p1: Point, p2: Point) -> bool {
    match p1 {
        Point::Infinity => match p2 {
            Point::Infinity => true,
            _ => false,
        },
        Point::ExistingPoint {x:x1, y:y1} => match p2 {
            Point::Infinity => false,
            Point::ExistingPoint {x:x2, y:y2} => (x1 == x2) && (y1 == y2),
        },
    }
}

pub fn is_infinity(p: Point) -> bool {
    match p {
        Point::Infinity => true,
        _ => false,
    }
}

pub fn sum_ponts(p1: Point, p2: Point) -> Point {
    p1
}