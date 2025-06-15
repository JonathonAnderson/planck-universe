use planck_universe::units::fundamental::{ Speed, Point, Moment };
use planck_universe::units::temporal::Duration;
use planck_universe::units::geometric::Line;
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
}