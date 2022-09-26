// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    let x = 9;
    let y = 9;
    println!("{x}......{y}");

    println!("Coverting from Farienhiet to  Celsius");
    fahrenheit_to_celcius(3);

    println!("Fibonacci numbers");
    fibonacci();
}

// Convert temperatures between Fahrenheit and Celsius.
const CONVERSION: i32 = 5;
fn fahrenheit_to_celcius(num: i32) {
    let ans: i32 = num * CONVERSION;

    println!(" temp value in celcius is {ans}")
}

fn fibonacci() {
    let mut fib_prev = 0;
    let mut fib_curr = 1;

    for i in 0..100 {
        let fib_next = fib_prev + fib_curr;
        fib_prev = fib_curr;
        fib_curr = fib_next;

        // println!("Value of fib_prev: {}", fib_prev);
        println!("prev fib is {fib_curr} the next fib is {fib_next}");
        // println!("prev fib is {:} the next fib is {:}", fib_curr, fib_next);

        if i == 10 {
            break;
        }
    }
}
