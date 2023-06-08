fn main() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Slices: {hello} {world} !!!");

    let s2 = "hello world";

    let w1 = first_word(&s);
    let w2 = first_word(s2);

    println!("{w1}");
    println!("{w2}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[0..3];

    for item in slice {
        println!("{item}");
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
