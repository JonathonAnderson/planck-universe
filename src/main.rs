use planck_universe::units::Speed;

pub fn main() -> () {
    let speed : Speed = Speed::new(0.0);
    
    println!("{:#?}", speed);
    println!("{:#?}", speed.set_speed( 99.0_f64 * 10.0_f64.powi(-2)).unwrap());
}