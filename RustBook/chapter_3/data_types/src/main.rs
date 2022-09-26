use std::io;

fn main() {
    println!("Rust Data Types!");
    println!("Rust Scalar and Compound Types!");
    println!();
    // Understanding scalar types
    // Rust has four scalar types : Integers | floats | string | boolean

    println!("Integers");
    working_with_intergers();
    //Working with floats
    println!();
    println!("Working with Floats ");
    working_with_floats();

    println!();
    println!("Working with char ");
    // working_with_floats();

    println!();
    println!("Working with Coumpond types | array and floats ");
    // working_with_floats();

    println!();
    println!("Numerical Operations");
    numerical_operations();

    println!();
    println!("Tuples and tuple unpacking");
    tuples();

    println!();
    println!("working with arrays ");
    // array_index();
    match_case();
}

fn working_with_intergers() {
    let x: u8 = u8::pow(2, 7) - 1;
    println!("the value of x is {}", x);
    // let y: i8 = i8::pow(2, 8)-1;
    // println!("the value of y is {}",y);
    let z: u32 = u32::pow(2, 31) - 1;
    println!("the value of z is {}", z);
    // let a: i32 = i32::pow(2, 32);
    // println!("the value of a is {}",a);
    let b: u64 = u64::pow(2, 63) - 1;
    println!("the value of b is {}", b);
    // let c: i64 = 45;
    // println!("the value of c is {}",c);
}
fn working_with_floats() {
    let d = 2.0; // f64

    let f: f32 = 3.0; // f32
    println!("the floating values are {} {}", d, f)
}

fn numerical_operations() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;
    println!("the sum{} the difference {} the product {} the quotient {} the floored {} the remainder {},",sum,difference,product,quotient,floored,remainder);
}

fn tuples() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let (five_00, six_4, one_1) = x;

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!(
        "tuple x = {:#?} with {} at index 0, {} at index 1 and {} at index 2 ",
        x, five_hundred, six_4, one
    );
}

fn array_index() {
    let arr = [1, 2, 3, 4, 5];

    for val in arr {
        println!( "{}", val);

    }

    let my_string = String::from("hello world");

    // for letter in my_string.chars() {
    //     println!("{}", letter);

    // }

    for (x,letter2) in my_string.chars().enumerate() {
        println!("index{} , value {}",x, letter2);




    println!("Please enter an array index.");

    // let arr :[f64, u8 , i32] = [35.3, 3, -10];// not possible 

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}}


fn match_case(){
    let mut some_vector = vec![1, 2, 3, 4];
    for i in 0..20{
        let m = match some_vector.pop(){
            Some(val) => "got_a_value".to_string(),
            None => "none".to_string(),

        };
        println!("{m}")
    }
}


