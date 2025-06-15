use planck_universe::physics::motion::*;
use planck_universe::physics::temporal::*;
use planck_universe::physics::geometric::*;
use planck_universe::Object;
use num_bigint::BigInt;

pub fn main() -> () {
    let speed0: Speed =  Speed::new(0.0);

    println!("{:#?}", speed0);

    enum Axis {
        X = 0,
        Y = 1,
        Z = 2,
    }

    let mut point0_vec: Vec<BigInt> = Vec::new();
    point0_vec.insert(Axis::X as usize, BigInt::from(18));
    point0_vec.insert(Axis::Y as usize, BigInt::from(34));
    point0_vec.insert(Axis::Z as usize, BigInt::from(72));
    let mut point1_vec: Vec<BigInt> = Vec::new();
    point1_vec.insert(Axis::X as usize, BigInt::from(37));
    point1_vec.insert(Axis::Y as usize, BigInt::from(91));
    point1_vec.insert(Axis::Z as usize, BigInt::from(4));

    let point0: Point =  Point::new(point0_vec);
    let point1: Point =  Point::new(point1_vec);

    println!("{:#?}", point0);

    let moment0: Moment = Moment::new(BigInt::from(45));
    let moment1: Moment = Moment::new(BigInt::from(60));

    println!("{:#?}", moment0);
    println!("{:#?}", moment1);

    let duration0: Duration = Duration::new(&moment0, &moment1);

    println!("{:#?}", duration0);

    let line0: Line = Line::new(&point0, &point1);

    println!("{:#?}", line0);

    let object0: Object = Object::new(&moment0, &moment1, &vec![point0, point1], &speed0);

    println!("{:#?}", object0);
}