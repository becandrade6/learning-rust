use std::io;

fn main() {
    loop {
        println!("------------------------");
        println!("Welcome to the temperature converter tool");
        println!("You can convert °C or °F into one another.");
        println!("Type C to convert from Celsius or F to convert from Fahrenheit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "C" => {
                let value: f64 = get_typed_value();

                let fahrenheit_value = convert_celsius_to_fahrenheit(value);

                println!("{value}°C in Fahrenheit is: {fahrenheit_value:.2}°F");
            }
            "F" => {
                let value: f64 = get_typed_value();

                let celsius_value = convert_fahrenheit_to_celsius(value);

                println!("{value}°F in Celsius is: {celsius_value:.2}°C");
            }
            _ => {
                println!("You did not type only C or F for the choice option.");
                println!("Try again!");
                continue;
            }
        }
    }
}

fn get_typed_value() -> f64 {
    loop {
        println!("Type the value with digits only. Example: 66.6");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value");

        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Failed to read the entered value, try again...");
            }
        }
    }
}

fn convert_celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    (celsius_temp * 9.0 / 5.0) + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit_temp: f64) -> f64 {
    (fahrenheit_temp - 32.0) * 5.0 / 9.0
}
