use crate::models::TemperatureConversionOption;
use std::io::{self, Write};

pub fn get_user_input() {
    loop {
        print_menu();

        let option = match read_input().parse::<u32>() {
            Ok(choice) => TemperatureConversionOption::from_u32(choice),
            Err(_) => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };

        if let TemperatureConversionOption::Exit = option {
            break;
        } else if let TemperatureConversionOption::Invalid = option {
            println!("Invalid option. Please try again.");
            continue;
        };

        println!("Enter the temperature value: ");
        match read_input().parse::<f64>() {
            Ok(value) => {
                let temperature = option.convert_temperature(value);
                temperature.display();
            }
            Err(_) => {
                println!("Invalid temperature value. Please try again.");
                continue;
            }
        };
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_menu() {
    println!("\nPlease Choose an Option\n");
    println!("1. Celsius to Fahrenheit");
    println!("2. Celsius to Kelvin");
    println!("3. Fahrenheit to Celsius");
    println!("4. Fahrenheit to Kelvin");
    println!("5. Kelvin to Celsius");
    println!("6. Kelvin to Fahrenheit");
    println!("7. Exit\n");
    println!("Enter your choice: ");
}
