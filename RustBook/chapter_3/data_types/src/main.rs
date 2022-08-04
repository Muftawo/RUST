fn main() {
    println!("Rust Data Types!");
    println!("Rust Scalar and Compound Types!");

    // Understanding scalar types
    // Rust has four scalar types : Integers | floats | string | boolean

    println!("Integers");
    let x: u8 = u8::pow(2, 7)-1 ;
    println!("the value of x is {}",x);
    // let y: i8 = i8::pow(2, 8)-1;
    // println!("the value of y is {}",y);
    let z: u32 = u32::pow(2, 31) - 1;
    println!("the value of z is {}",z);
    // let a: i32 = i32::pow(2, 32);
    // println!("the value of a is {}",a);
    let b: u64 = u64::pow(2, 63) - 1;
    println!("the value of b is {}",b);
    // let c: i64 = 45;
    // println!("the value of c is {}",c);


fn array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
