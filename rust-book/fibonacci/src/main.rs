use std::io;
use std::io::Write;

fn fibonacci(nth: i32) -> i32 {
    if nth <= 0 {
        panic!("Wrong number")
    }
    // info: https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html#section1.2.1
    match nth {
        1 => 0,
        2 => 1,
        _ => fibonacci(nth - 1) + fibonacci(nth - 2),
    }
}

fn user_input_int() -> i32 {
    loop {
        println!("Please type in a number");
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => print!(""),
            Err(_) => continue,
        }
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Fail");
        let number: i32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("{:?} is not a valid number", number.trim());
                continue;
            }
        };
        return number;
    }
}

fn main() {
    let nth = user_input_int();
    let fibonacci_number = fibonacci(nth);
    println!("The {}th fibonacci number is: {}", nth, fibonacci_number)
}
