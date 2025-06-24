use planck_universe::physics::temporal::*;
use planck_universe::physics::spatial::*;
use planck_universe::physics::motion::*;
use num_bigint::BigInt;
use rust_decimal::Decimal;

pub fn main() -> () {
    //
    // Motion
    //
    let speed: Speed = Speed::new_negative_normalized(Decimal::new(4375, 4));
    let speed_neg: Speed = Speed::new_negative_normalized(Decimal::new(-4375, 4));
    println!("A speed, in negative normalized form where 0.0 is no motion and 1.0 is the speed of light.");
    println!("Negative values just for fun");
    println!("{:#?}", speed);
    println!("{:#?}", speed_neg);

    // let _velocity = todo!();

    //
    // Spatial
    //
    enum Axis {
        X = 0,
        Y = 1,
        Z = 2,
    }

    // Line
    let mut point_0_vec: Vec<BigInt> = Vec::new();
    point_0_vec.insert(Axis::X as usize, BigInt::from(18));
    point_0_vec.insert(Axis::Y as usize, BigInt::from(34));
    point_0_vec.insert(Axis::Z as usize, BigInt::from(72));
    let mut point_1_vec: Vec<BigInt> = Vec::new();
    point_1_vec.insert(Axis::X as usize, BigInt::from(37));
    point_1_vec.insert(Axis::Y as usize, BigInt::from(91));
    point_1_vec.insert(Axis::Z as usize, BigInt::from(4));

    let point_0: Point =  Point::new(&point_0_vec);
    let point_1: Point =  Point::new(&point_1_vec);
    println!("A point, the most fundamental spatial unit");
    println!("{:#?}", point_0);

    let line_0: Shape = Shape::new(&vec![point_0.clone(), point_1.clone()]);
    println!("A line, which is a shape made from two points");
    println!("{:#?}", line_0);

    // Polygon
    let mut point_2_vec: Vec<BigInt> = Vec::new();
    point_2_vec.insert(Axis::X as usize, BigInt::from(76));
    point_2_vec.insert(Axis::Y as usize, BigInt::from(46));
    point_2_vec.insert(Axis::Z as usize, BigInt::from(11));
    let mut point_3_vec: Vec<BigInt> = Vec::new();
    point_3_vec.insert(Axis::X as usize, BigInt::from(95));
    point_3_vec.insert(Axis::Y as usize, BigInt::from(62));
    point_3_vec.insert(Axis::Z as usize, BigInt::from(84));

    let point_2: Point =  Point::new(&point_2_vec);
    let point_3: Point =  Point::new(&point_3_vec);

    let shape_0_points: Vec<Point> = vec![point_0, point_1, point_2, point_3];
    let shape_0: Shape = Shape::new(&shape_0_points);
    println!("A poloygon, formed from four points");
    println!("{:#?}", shape_0);

    //
    // Temporal
    //
    let moment_0: Moment = Moment::new(BigInt::from(45));
    let moment_1: Moment = Moment::new(BigInt::from(60));
    let moment_2: Moment = Moment::new(BigInt::from(1076));
    let moment_3: Moment = Moment::new(BigInt::from(3234));
    println!("A moment, the most fundamental temporal unit");
    println!("{:#?}", moment_0);

    let duration_0: Duration = Duration::new(&moment_0, &moment_1);
    let duration_1: Duration = Duration::new(&moment_2, &moment_3);
    println!("A duration, a single measure of one moment to another");
    println!("{:#?}", duration_0);

    let period_0: Period = Period::new(&duration_0, &duration_1);
    println!("A period, two durations where one duration is thought to be a response to the other (causally related)");
    println!("Note that this allows irregular and asynchronous periods");
    println!("{:#?}", period_0);
}