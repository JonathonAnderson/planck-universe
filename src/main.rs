use planck_universe::physics::motion::*;
use planck_universe::physics::temporal::*;
use planck_universe::physics::geometric::*;
use planck_universe::Object;
use num_bigint::BigInt;

pub fn main() -> () {
    let speed_0: Speed =  Speed::new(0.0);

    println!("{:#?}", speed_0);

    enum Axis {
        X = 0,
        Y = 1,
        Z = 2,
    }

    let mut point_0_vec: Vec<BigInt> = Vec::new();
    point_0_vec.insert(Axis::X as usize, BigInt::from(18));
    point_0_vec.insert(Axis::Y as usize, BigInt::from(34));
    point_0_vec.insert(Axis::Z as usize, BigInt::from(72));
    let mut point_1_vec: Vec<BigInt> = Vec::new();
    point_1_vec.insert(Axis::X as usize, BigInt::from(37));
    point_1_vec.insert(Axis::Y as usize, BigInt::from(9_1));
    point_1_vec.insert(Axis::Z as usize, BigInt::from(4));

    let point_0: Point =  Point::new(point_0_vec);
    let point_1: Point =  Point::new(point_1_vec);

    println!("{:#?}", point_0);

    let moment_0: Moment = Moment::new(BigInt::from(45));
    let moment_1: Moment = Moment::new(BigInt::from(6_0));

    println!("{:#?}", moment_0);
    println!("{:#?}", moment_1);

    let duration_0: Duration = Duration::new(&moment_0, &moment_1);

    println!("{:#?}", duration_0);

    let line_0: Line = Line::new(&point_0, &point_1);

    println!("{:#?}", line_0);

    // This is a point moving at zero speed and frozen in time
    let point_object_0: Object = Object::new(&moment_0, &moment_1, &vec![point_0.clone()], &speed_0);

    println!("{:#?}", point_object_0);
    // This is a line moving at zero speed and frozen in time
    let dimension_1_line_0: Object = Object::new(&moment_0, &moment_1, &vec![point_0.clone(), point_1.clone()], &speed_0);

    println!("{:#?}", dimension_1_line_0);
}