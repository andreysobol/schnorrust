use std::ops::Add;
use std::ops::Mul;

extern crate num_bigint;
use num_bigint::{BigInt, ToBigInt};

use secp256k1::{secp256k1_params};

use tool::unsigned_modulo;

pub enum Point{
    Infinity,
    ExistingPoint{
        x: BigInt,
        y: BigInt,
    }
}

impl Point{
    pub fn from_x(x: BigInt) -> Point{
        let sparam = secp256k1_params();
        let p = sparam.p;

        let one = 1.to_bigint().unwrap();
        let two = 2.to_bigint().unwrap();
        let three = 3.to_bigint().unwrap();
        let four = 4.to_bigint().unwrap();
        let seven = 7.to_bigint().unwrap();

        if x >= p {
            return Point::Infinity
        }
        
        let square = (x.modpow(&three, &p) + &seven) % &p;
        let y = square.modpow(&((&p + &one) / &four), &p);

        if y.modpow(&two, &p) != square {
            return Point::Infinity;
        }

        Point::ExistingPoint{
            x: x,
            y: y,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Point::Infinity => [].to_vec(),
            Point::ExistingPoint {x:x, y:y} => {

                let mut xv = x.to_bytes_be().1;
                xv.resize_with(32, || { 0 });

                let mut yv = y.to_bytes_be().1;
                yv.resize_with(32, || { 0 });

                let mut res = Vec::new();
                res.extend(xv);
                res.extend(yv);
                res
            },
        }
    }
}

impl Add for Point{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        sum_points(&self, &other)
    }
}

impl Mul<BigInt> for Point {
    type Output = Self;

    fn mul(self, other: BigInt) -> Self::Output {
        mul_points(self, &other)
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

pub fn square(point: &Point) -> bool {
    if(is_infinity(point)){
        return false;
    }

    let sparam = secp256k1_params();
    let p = sparam.p;

    let one = 1.to_bigint().unwrap();
    let two = 2.to_bigint().unwrap();

    let y = get_y(point);

    let h = (&p - &one) / two;

    return y.modpow(&h, &p) == one;
}

//https://en.wikipedia.org/wiki/Elliptic_curve#The_group_law
pub fn sum_points(p1: &Point, p2: &Point) -> Point {

    let sparam = secp256k1_params();
    let p = sparam.p;
    let pm2 = &p - 2.to_bigint().unwrap();

    if is_infinity(&p1) && is_infinity(&p2){
        return Point::Infinity
    }

    if is_infinity(&p1){
        return match p2 {
            Point::ExistingPoint {x, y} => Point::ExistingPoint {x:x.clone(), y:y.clone()},
            Point::Infinity => panic!("Point is Infinity")
        }
    }
    if is_infinity(&p2){
        return match p1 {
            Point::ExistingPoint {x, y} => Point::ExistingPoint {x:x.clone(), y:y.clone()},
            Point::Infinity => panic!("Point is Infinity")
        }
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

    let mut xr = (&s * &s - get_x(&p1) - get_x(&p2));
    xr = unsigned_modulo(&xr, &p);

    let mut yr = (&s * (get_x(&p1) - &xr) - get_y(&p1));
    yr = unsigned_modulo(&yr, &p);

    let rp = Point::ExistingPoint{
        x: xr,
        y: yr,
    };

    return rp
}

pub fn mul_points(p: Point, n: &BigInt) -> Point {
    let one = 1.to_bigint().unwrap();
    let zero = 0.to_bigint().unwrap();
    let mut ni = n.clone();
    let mut res = Point::Infinity;
    let mut current = p;

    for item in 0..256 {
        let c = &ni & &one;
        if (c>zero){
            res = sum_points(&res, &current);
        }
        if item == 255 {
            return res;
        } else {
            current = sum_points(&current, &current);
            ni = ni >> 1;
        }

    }

    panic!("Something went wrong in cycle on mul_points");
    Point::Infinity
}

#[test]
fn test_sum_points() {

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

    let res = sum_points(&p1, &p2);

    let expectx = BigInt::parse_bytes(b"C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5", 16).unwrap();
    let expecty = BigInt::parse_bytes(b"1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A", 16).unwrap();

    assert_eq!(&expectx, get_x(&res));
    assert_eq!(&expecty, get_y(&res));
}

#[test]
fn test_mul_points() {
    let px = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let py = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();

    let p = Point::ExistingPoint{
        x: px,
        y: py,
    };

    let expectx = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let expecty = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();

    let res = mul_points(p, &1.to_bigint().unwrap());

    assert_eq!(&expectx, get_x(&res));
    assert_eq!(&expecty, get_y(&res));

}

#[test]
fn test_mul_points2() {
    let px = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let py = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();

    let j = BigInt::parse_bytes(b"B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF", 16).unwrap();

    let p = Point::ExistingPoint{
        x: px,
        y: py,
    };

    let expectx = BigInt::parse_bytes(b"DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659", 16).unwrap();
    let expecty = BigInt::parse_bytes(b"2CE19B946C4EE58546F5251D441A065EA50735606985E5B228788BEC4E582898", 16).unwrap();

    let res = mul_points(p, &j);

    assert_eq!(&expectx, get_x(&res));
    assert_eq!(&expecty, get_y(&res));
}