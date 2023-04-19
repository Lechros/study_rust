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

fn if_expr() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("condition was false");
    }

    // you can
    let x = if number == 3 { 5 } else { 4 };
}

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn for_each() {
    let a = [10, 20, 30, 40, 50];

    for el in a {
        println!("The value is: {el}");
    }

    for x in 0..10 {
        println!("{x}");
    }
}

fn nested_loop() {
    let x= 'outer: loop {
        for y in 0..5 {
            if y > 2 {
                break 'outer; // break label even works with non-loop blocks
            }
        }
    };
}