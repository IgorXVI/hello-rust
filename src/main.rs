fn main() {
    //Ownership rules
    //1. Each value in Rust has a variable that's called its owner.
    //2. There can only be one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }
    // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Moved s1 to s2, s1 is no longer valid

    //println!("{}", s1);

    let s3 = s2.clone(); // Cloned s2, so s2 and s3 are valid

    println!("{s2}");
    println!("{s3}");
}
