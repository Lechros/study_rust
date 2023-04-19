fn main() {
    let y = {
        let x = 3;
        x + 1
    }; // last expression of block is returned.

    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("five() is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    let x = 5;
    x // last expression should NOT end with semicolon, or it is statement.
}