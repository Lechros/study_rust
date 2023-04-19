use std::{cmp::Ordering, io};
/*
 * Trait in use must be in scope.
 */
use rand::Rng;

fn main() {
    println!("Guess the number!");

    /*
     * 'start..=end': range expression. == [start, end)
     */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    /*
     * 'loop' creates an infinite loop.
     */
    loop {
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
         * redeclare = variable name shadowing
         * 'read_line' includes newline character, remove it with 'trim'.
         * annotating 'guess' as 'u32' makes 'parse' into 'parse<u32>',
         * and even 'secret_number' into 'u32' because it is cmp-ed with guess later!
         */
        let guess: u32 = match guess.trim().parse() { // match parse 'Result'
            Ok(num) => num, // returns the value if Ok
            Err(_) => continue, // continues outer loop if Err, '_' match all Err values
        };

        /*
         * string interpolation for free.
         */
        println!("You guessed: {guess}");

        /*
         * ('match' like pattern matching)
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {// autocomplete works for last enum!
                println!("You win!");
                break; // breaks outer loop. So good better than switch!
            }
        }
    }
}
