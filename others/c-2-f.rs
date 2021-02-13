// 摄氏度和华氏度之间的转换器
use std::io;

fn main() {
    loop {
        println!("Which one will you input? Fahrenheit degree or Celsius degree?\n1. Fahrenheit degree\n2. Celsius degree");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        let choice: u64 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[!] Sorry, not a valid number");
                continue;
            }
        };
        // println!("Your input: {}", choice);
        let mut degree = String::new();
        if choice == 1 {
            println!("Please input Fahrenheit degree(°F)");
        } else if choice == 2 {
            println!("Please input Celsius degree(°C)");
        }
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line.");
        let degree: f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[!] Sorry, not a valid number");
                continue;
            }
        };
        if choice == 1 {
            println!("Fahrenheit degree: {}°F", degree);
            println!("Celsius degree: {}°C", (degree - 32.0) / 1.8);
        } else if choice == 2 {
            println!("Celsius degree: {}°C", degree);
            println!("Fahrenheit degree: {}°F", degree * 1.8 + 32.0);
        }
    }
}
