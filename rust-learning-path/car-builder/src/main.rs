// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

// Build a "Car" by using valumotores from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if convertible {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", transmission, color, miles)
        }
    }
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        motor: transmission,
        roof: convertible,
        age: car_quality(miles),
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

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used
}

// #[derive(PartialEq, Debug)]
// enum Colors {
//     Blue(&str),
//     Green(&str),
//     Red(&str),
//     Silver(&str)
// }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles)
    } else {
        return (Age::New, 0)
    }
}

fn main() {
    let colors:[&str; 4] = ["blue", "green", "red", "silver"];
    let mut car = car_factory("red".to_string(), Transmission::Automatic, false, 50000);
    let mut engine: Transmission = Transmission::Manual;
    
    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    
    // println!("Color: {}\nTransmission type: {:?}\nConvertible: {} \nAge: {:?}", car.color, car.transmission, car.convertible, car.age);
}
