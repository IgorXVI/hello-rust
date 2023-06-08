fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 11 {
            break counter;
        }
    };

    println!("The result is {result}!");

    let mut number = 10;

    while number > 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {element}");
    }

    for number in 1..4 {
        println!("the range value is: {number}");
    }
}
