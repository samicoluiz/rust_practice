// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic
}

fn main() {
    let night_rider = car_factory("red".to_string(), Transmission::Automatic, false);
    println!("Car color: {}\nTransmission type: {:?}\nConvertible: {}", night_rider.color, night_rider.transmission, night_rider.convertible);
}
