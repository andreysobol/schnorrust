extern crate num_bigint;
use num_bigint::{BigUint, ToBigUint};

use secp256k1::{secp256k1_params};

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

pub fn is_infinity(p: &Point) -> bool {
    match p {
        Point::Infinity => true,
        _ => false,
    }
}

pub fn get_x(p: &Point) -> &BigUint {
    match p {
        Point::ExistingPoint {x, y} => x,
        Point::Infinity => panic!("Point is Infinity")
    }
}

pub fn get_y(p: &Point) -> &BigUint {
    match p {
        Point::ExistingPoint {x, y} => y,
        Point::Infinity => panic!("Point is Infinity")
    }
}

//https://en.wikipedia.org/wiki/Elliptic_curve#The_group_law
pub fn sum_ponts(p1: Point, p2: Point) -> Point {

    let sparam = secp256k1_params();
    let p = sparam.p;
    let pm2 = &p - 2.to_biguint().unwrap();

    if is_infinity(&p1){
        return p2
    }
    if is_infinity(&p2){
        return p1
    }
    if (get_x(&p1) == get_x(&p2)) && (get_y(&p1) != get_y(&p2)) {
        return Point::Infinity
    }

    let numerator = get_y(&p1) - get_y(&p2);
    let denominator = (get_x(&p1) - get_x(&p2)).modpow(&pm2, &p);
    let s = (numerator * denominator) % &p;
    let xr = (&s * &s - get_x(&p1) - get_x(&p2)) % &p;
    let yr = get_y(&p1) + s * (&xr - get_x(&p1));
    let rp = Point::ExistingPoint{
        x: xr,
        y: (-yr),
    };

    return rp
}