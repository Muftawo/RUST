fn main() {
    let number = 3;
    // shadow number to 7
    let number= number + 4 ;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!();
    println!("Multiple conditions");
    multiple_conditions();

    println!();
    println!("if in let statement ");
    if_let_expressions() ;
}


fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

