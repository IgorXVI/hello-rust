fn main() {
    //The Rules of References
    //
    //1. At any given time, you can have either one mutable reference or
    //any number of immutable references.
    //
    //2. References must point to valid data.

    let s1 = String::from("cabeça de cavalo");
    let len = calculate_length(&s1);

    println!("The length of \"{s1}\" is {len}");
}

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}
