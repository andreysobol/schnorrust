extern crate num_bigint;
use num_bigint::{BigInt, ToBigInt};

use secp256k1::{secp256k1_params};

pub enum Point{
    Infinity,
    ExistingPoint{
        x: BigInt,
        y: BigInt,
    }
}

pub fn equal_points(p1: &Point, p2: &Point) -> bool {
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

pub fn get_x(p: &Point) -> &BigInt {
    match p {
        Point::ExistingPoint {x, y} => x,
        Point::Infinity => panic!("Point is Infinity")
    }
}

pub fn get_y(p: &Point) -> &BigInt {
    match p {
        Point::ExistingPoint {x, y} => y,
        Point::Infinity => panic!("Point is Infinity")
    }
}

//https://en.wikipedia.org/wiki/Elliptic_curve#The_group_law
pub fn sum_ponts(p1: Point, p2: Point) -> Point {

    let sparam = secp256k1_params();
    let p = sparam.p;
    let pm2 = &p - 2.to_bigint().unwrap();

    if is_infinity(&p1){
        return p2
    }
    if is_infinity(&p2){
        return p1
    }
    if (get_x(&p1) == get_x(&p2)) && (get_y(&p1) != get_y(&p2)) {
        return Point::Infinity
    }

    let mut numerator;
    let mut denominator;

    if equal_points(&p1, &p2){
        numerator = 3*get_x(&p1)*get_x(&p1);
        denominator = (get_y(&p1)+get_y(&p1)).modpow(&pm2, &p);
    } else {
        numerator = get_y(&p1) - get_y(&p2);
        denominator = (get_x(&p1) - get_x(&p2)).modpow(&pm2, &p);
    }

    let s = (numerator * denominator) % &p;

    let ds = s.to_str_radix(16);

    let xr = (&s * &s - get_x(&p1) - get_x(&p2)) % &p;
    let yr = get_y(&p1) + s * (&xr - get_x(&p1));
    let rp = Point::ExistingPoint{
        x: xr,
        y: (-yr),
    };

    return rp
}

#[test]
fn test_sum_ponits() {
    assert_eq!(2 + 2, 4);

    let gx: BigInt = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let gy: BigInt = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();

    let p1 = Point::ExistingPoint{
        x: gx.clone(),
        y: gy.clone(),
    };

    let p2 = Point::ExistingPoint{
        x: gx,
        y: gy,
    };

    let res = sum_ponts(p1, p2);

    let expectx = BigInt::parse_bytes(b"C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5", 16).unwrap();
    let expecty = BigInt::parse_bytes(b"1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A", 16).unwrap();

    assert_eq!(&expectx, get_x(&res));
}