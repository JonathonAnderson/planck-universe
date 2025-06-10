use planck_universe::units::*;
use num_bigint::BigInt;

pub fn main() -> () {
    let speed: Speed =  Speed::new(-99E-2);

    println!("{:#?}", speed);

    let point: Point =  Point::new(BigInt::from(34));

    println!("{:#?}", point);

    let moment: Moment =  Moment::new(BigInt::from(45));

    println!("{:#?}", moment);
}