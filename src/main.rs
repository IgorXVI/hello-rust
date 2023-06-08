fn main() {
    //Ownership rules
    //1. Each value in Rust has a variable that's called its owner.
    //2. There can only be one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.

    let s = String::from("hello");
    takes_ownership(s); //s moved to the scope of the function takes_ownership, s is no longer valid

    //the following line will result in a compilation error "borrow of moved value"
    //println!("{s}");

    let x = 10;
    makes_copy(10); // primitive values are copied by default
    println!("{x}");

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {s1}, s3 = {s3}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
