use planck_universe::physics::temporal::*;
use planck_universe::physics::spatial::*;
use planck_universe::Object;
use num_bigint::BigInt;

pub fn main() -> () {
    //
    // Motion
    //
    let speed_0_5: Speed = Speed::new(0.5);

    //
    // Geometrics
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

    let point_0: Point =  Point::new(point_0_vec);
    let point_1: Point =  Point::new(point_1_vec);

    let _line_0: Line = Line::new(&point_0, &point_1);

    // Shape
    let mut point_2_vec: Vec<BigInt> = Vec::new();
    point_2_vec.insert(Axis::X as usize, BigInt::from(76));
    point_2_vec.insert(Axis::Y as usize, BigInt::from(46));
    point_2_vec.insert(Axis::Z as usize, BigInt::from(11));
    let mut point_3_vec: Vec<BigInt> = Vec::new();
    point_3_vec.insert(Axis::X as usize, BigInt::from(95));
    point_3_vec.insert(Axis::Y as usize, BigInt::from(62));
    point_3_vec.insert(Axis::Z as usize, BigInt::from(84));

    let point_2: Point =  Point::new(point_2_vec);
    let point_3: Point =  Point::new(point_3_vec);

    let shape_0_points: Vec<Point> = vec![point_0.clone(), point_1.clone(), point_2.clone(), point_3.clone()];
    let _shape_0: Shape = Shape::new(shape_0_points);

    //
    // Temporal
    //
    let moment_0: Moment = Moment::new(BigInt::from(45));
    let moment_1: Moment = Moment::new(BigInt::from(60));
    let moment_2: Moment = Moment::new(BigInt::from(1076));
    let moment_3: Moment = Moment::new(BigInt::from(3234));

    let duration_0: Duration = Duration::new(&moment_0, &moment_1);
    let duration_1: Duration = Duration::new(&moment_2, &moment_3);

    let _period_0: Period = Period::new(&duration_0, &duration_1);

    //
    // Objects
    //
    println!("This is a null point moving at zero speed");
    let null_point_0: Object = Object::new();
    println!("{:#?}", null_point_0);

    println!("This is a vector with a point but no other properties");
        let spatial_point_0: Object = Object::new();
        let spatial_point_0: Object = Object::set_geometry(spatial_point_0,vec![point_0, point_1]);
    println!("{:#?}", spatial_point_0);

    println!("This is a point moving at zero speed and exists for a fix duration");
    let point_object_0: Object = Object::new();
    let point_object_0 = point_object_0.set_t_0(&moment_0);
    let point_object_0 = point_object_0.set_t_final(&moment_1);
    println!("{:#?}", point_object_0);

    println!("This is a line moving at non-zero speed and exists for a fixed duration");
    let one_dimensional_line_0: Object = Object::new();
    let one_dimensional_line_0 = one_dimensional_line_0.set_speed(speed_0_5);
    let one_dimensional_line_0 = one_dimensional_line_0.set_t_0(&moment_2);
    let one_dimensional_line_0 = one_dimensional_line_0.set_t_final(&moment_3);
    println!("{:#?}", one_dimensional_line_0);

    println!("This is a mult-dimensonal moving at zero speed and exists for a fixed duration");
    let multi_dimensional_object_0: Object = Object::new();
    let multi_dimensional_object_0 = multi_dimensional_object_0.set_t_0(&moment_2);
    let multi_dimensional_object_0 = multi_dimensional_object_0.set_t_final(&moment_3);
    println!("{:#?}", multi_dimensional_object_0);

    println!("A 3 Dimensional object moving at a constant normalized speed");
    let three_dimensional_constant_speed_0: Object = Object::new();
    println!("{:#?}", three_dimensional_constant_speed_0);

    println!("An oriented 3D object moving at zero speed");
    let three_dimensional_constant_speed_0: Object = Object::new();
    println!("{:#?}", three_dimensional_constant_speed_0);

}