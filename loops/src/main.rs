fn main() {
    let mut contador = 0;
    let result = 'result: loop {
        println!("again");
        contador += 1;
        if contador >= 10 {
            break 'result contador * 2;
        }
    };

    println!("The result is {result}");

    let a = [10,20,30,40,50];
    let mut index = 0;

    for element in a {
        println!("the value is: {}", element);

        index += 1;
    }

    for number in (1..4) {
        println!("{number}");
    }
    println!("go!")
}
