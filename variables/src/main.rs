fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
    // OR
    {
        let x = 5; // 5
        let x = x + 1; // 6
        {
            let x = x * 2; // 12
            println!("The value of x in the inner scope is: {x}"); // 12
        }
        println!("The value of x is: {x}"); // 6
    }
    /*
     * use shadowing to change variable's type,
     * but mut cannot mutate type
     */


    /*
     * i8, i16, i32, i64, i128, isize (i32 for 32bit system, i64 for 64bit system)
     * u8, ...
     */
    let integer: i8;

    let decimal_literal = 9__8_222_i64;
    let hex_literal = 0xfF;
    let octal_literal = 0o77;
    let binary_literal = 0b1111_0000;
    let byte_literal = b'A'; // u8 only

    /*
     * panics in debug mode
     * becomes 2s complement in release mode (still an error, use 'wrapping_*' instead)
     */
    let integer_overflow = i32::MAX + 1;
    i32::wrapping_add(i32::MAX, 1); // safely wraps to 2s complement
    i32::checked_add(i32::MAX, 1); // returns 'None' if overflow
    i32::overflowing_add(i32::MAX, 1); // returns (value, bool=overflowed) tuple
    i32::saturating_add(i32::MAX, 1); // bounds value to type's limit

    /*
     * default is f64(double)
     */
    let x = 2.0;
    let y: f32 = 3.0;

    // boolean types
    let t = true;
    let f: bool = false;

    // char types, uses unicode and each is 4 bytes!
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // tuple (compound type)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = tup;
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let x = tup.0; // use index as name
    let y = tup.1;
    let z = tup.2;

    let unit = (); // = empty value / empty return type.

    // fixed-length array
    let a = [1, 2, 3, 4, 5];
    let b: [char; 3] = ['a', 'b', 'c'];
    let c = [6; 3]; // = [6, 6, 6]
    let value = a[0];
    let value2 = a[7]; // out of bound: will be covered in Chap. 9
}
