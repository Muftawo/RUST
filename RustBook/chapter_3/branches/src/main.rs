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

