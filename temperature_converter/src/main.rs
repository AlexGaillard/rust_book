use dialoguer::Select;
use std::io;

fn main() {
    loop {
        println!("Enter temperature");

        let mut temperature: String = String::new();
        let temperature_choices = vec!["Celsius", "Fahrenheit"];

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read temperature");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        let temperature_choice = Select::new()
            .with_prompt("What are you converting to?")
            .items(&temperature_choices)
            .interact()
            .unwrap();

        let temperature_letter = &temperature_choices[temperature_choice][0..1];

        let converted_temperature: f64;

        if temperature_letter == "F" {
            converted_temperature = (temperature as f64) * 1.8 + 32.0;
        } else {
            converted_temperature = (temperature as f64) - 32.0 / 1.8;
        }

        println!(
            "Your converted temperature: {}{}",
            converted_temperature, temperature_letter
        );

        break;
    }
}
