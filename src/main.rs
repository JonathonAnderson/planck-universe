use planck_universe::physics::motion::*;
use planck_universe::physics::temporal::*;
use planck_universe::physics::geometric::*;
use planck_universe::Object;
use num_bigint::BigInt;

pub fn main() -> () {
    let speed0: Speed =  Speed::new(0.0);

    println!("{:#?}", speed0);

    let point0: Point =  Point::new(vec![BigInt::from(34)]);
    let point1: Point =  Point::new(vec![BigInt::from(56)]);

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