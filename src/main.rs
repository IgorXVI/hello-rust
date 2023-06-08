fn main() {
    //The Rules of References
    //
    //1. At any given time, you can have either one mutable reference or
    //any number of immutable references.
    //
    //2. References must point to valid data.

    let mut s1 = String::from("cabeça de cavalo");

    change(&mut s1);

    println!("{s1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
