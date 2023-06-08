fn main() {
    //INTEGER

    let a = 98_222; //Decimal
    let b = 0xff; //Hex
    let c = 0o77; //Octal
    let d = 0b1111_0001; //Binary
    let e = b'A'; //Byte (u8 only)

    let f: u8 = 255;

    //Floating-point numbers

    let g = 2.0;
    let h: f32 = 3.0;

    //Tuples

    let tup = ("banana", 100_000);

    let (fruit, count) = tup;

    let count = tup.1;

    //Arrays

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    //8 values all set to zero
    let byte = [0; 8];

}
