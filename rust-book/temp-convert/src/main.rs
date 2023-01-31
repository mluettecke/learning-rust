use std::fmt;
use std::io;
use std::io::Write;

enum TempUnit {
    Fahrenheit,
    Celsius,
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Celsius => write!(f, "celsius"),
            Self::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

fn main() {
    println!("Pick what you want to do");
    println!("[1]: Convert Fahrenheit (°F) to Celsius (°C)");
    println!("[2]: Convert Celsius (°C) to Fahrenheit (°F)");

    let pick = read_option_from_user();

    match pick {
        1 => {
            let fahrenheit: f64 = read_temp_from_user(TempUnit::Fahrenheit);
            println!(
                "{} °F is {} °C",
                fahrenheit,
                fahrenheit_to_celsius(fahrenheit)
            )
        }
        2 => {
            let celsius: f64 = read_temp_from_user(TempUnit::Celsius);
            println!("{} °C is {} °F", celsius, celsius_to_fahrenheit(celsius))
        }
        _ => println!("Invalid pick"),
    }
}

fn read_option_from_user() -> i32 {
    loop {
        let mut pick = String::new();
        io::stdin().read_line(&mut pick).expect("Fail");
        let pick: i32 = match pick.trim().parse() {
            Ok(pick) => match pick {
                1 => 1,
                2 => 2,
                _ => {
                    println!("Try again");
                    continue;
                }
            },
            Err(_) => {
                println!("{:?} is not a valid pick", pick.trim());
                continue;
            }
        };
        return pick;
    }
}

fn read_temp_from_user(unit: TempUnit) -> f64 {
    loop {
        println!("Please give a value");
        print!("> ");

        match io::stdout().flush() {
            Ok(_) => print!(""),
            Err(_) => continue,
        }

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Fail");

        let temp: f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("{:?} is not a valid temperature in {}", temp.trim(), unit);
                continue;
            }
        };
        return temp;
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 0.555;
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return (celsius * 1.8) + 32.0;
}
