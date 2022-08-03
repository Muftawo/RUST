fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("Hello, world! {}", x);

    // Constants
    const NUM_OF_MINS_IN_A_WEEK: u32 = 7 * 24 * 60;
    println!("there are {} minutes in a week", NUM_OF_MINS_IN_A_WEEK);

    // Shadowing
    let y = 4;

    let y = y + y;
    {
        println!("Lets move to an inner scope and shadow Y");

        let y = y + 10;

        println!("Y in the inner scope is {}", y);
    }
    println!("The value of Y after shadowing is {}", y);

    let spaces = "          ";
    let spaces = spaces.len();

    println!("The lenght of spaces is {}", spaces)
}
