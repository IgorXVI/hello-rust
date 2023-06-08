fn main() {
    //The Rules of References
    //
    //1. At any given time, you can have either one mutable reference or
    //any number of immutable references.
    //
    //2. References must point to valid data.

    let mut s1 = String::from("cabeça de cavalo");

    let r1 = &s1;
    let r2 = &s1;

    //can only add mutable reference after the immutable references have been dropped

    //next line is invalid
    //let r3 = &mut s1;

    println!("r1: {r1}, r2: {r2}");

    //r1 and r2 are dropped

    //next line is valid
    let r3 = &mut s1;

    println!("r3: {r3}");
}
