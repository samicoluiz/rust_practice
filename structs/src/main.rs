fn main() {
    struct Person {
        name: String,
        age: u8,
        gender: char,
        profession: String,
    }

    let luiz = Person {
        name: String::from("Luiz Samico"),
        age: 35,
        gender: 'M',
        profession: String::from("programmer"),
    };

    println!("{}", luiz.age);

    let black = Colour(0, 0, 0);
    println!("{:#?}", black);
    dbg!(&black);
}

#[derive(Debug)]
struct Colour(i32, i32, i32);
