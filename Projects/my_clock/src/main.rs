
use chrono::Local;


// https://www.w3.org/TR/xml-entity-names/025.html
const DIGITS : [[&str; 11]; 5] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", "  ━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╻ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];





fn main() {
    println!("Time is An ILLUSION !");
    clock();
}


fn clock(){
    println!("\x1b[2J");
    print!("\x1b[?25l");

    loop{
        let t= Local::now();
        let time = t.format("%H:%M:%S").to_string();
        println!("time {:?}", time);
        for row in &DIGITS {
            for c in time.chars() {
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                println!("{}",col);
                println!("{}", row[col]);
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(999));
        print!("\x1b[7A");


    }
}
    
