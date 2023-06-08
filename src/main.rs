fn main() {
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("Number is {number}!");
}
