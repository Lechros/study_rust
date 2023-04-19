use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*
     * let: create immutable variable.
     * to make it mutable, use 'let mut'.
     * '=' assigns rh to lh.
     * String size auto grows.
     * use '::' to use associated function (method)
     */
    let mut guess = String::new();

    /*
     * can use 'std::io::stdin' instead of 'use std::io; io::stdin'.
     * returns instance of Stdin type.
     * '&': reference (same as C++)
     * use '&mut guess' instead of '&guess' to make mutable reference.
     */
    io::stdin()
        .read_line(&mut guess)
        /*
         * 'read_line' returns 'Result' value.
         * 'Result' is enum, can be in one of multiple states. (Ok and Err)
         * 'Result' should be handled (like Java)
         * 'expect' crashes program with argument msg if Err.
         *
         * I think expect is not a very good name for crashing...
         */
        .expect("Failed to read line");

    /*
     * string interpolation for free.
     */
    println!("You guessed: {guess}");
}
