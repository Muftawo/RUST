fn main() {
    loop {
        println!("again!");
        break;
    }

    println!();
    println!("Inner loops");
    inner_loop();

    println!();
    println!("store result from loop in a variable");
    result_from_loop();

    println!();
    println!("while loop");
    while_loop();

    println!();
    println!("for loop");
    for_loop();

    for_loop_reverse() ;
}

fn inner_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn result_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}


fn for_loop_reverse() {
    for number in (1..4).rev() {
        println!("rev{}!", number);
    }
    println!("LIFTOFF!!!");
}