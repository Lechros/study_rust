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
}
