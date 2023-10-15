use std::io;

fn main() {
    println!("Welcome to my temperature converter.");

    loop {
        println!("\nPlease input your temperature. (20F or -6.67C)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim().to_lowercase();
        if input.contains('q') {
            break;
        }

        let temp: String = input
            .chars()
            .filter(|c| c.is_digit(10) || *c == '-' || *c == '.')
            .collect();
        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Please input a valid temperature.");
                continue;            
            },
        };

        if input.contains('c') {
            println!("{:.2}°F", celsius_to_fahrenheit(temp));
        } else if input.contains('f') {
            println!("{:.2}°C", fahrenheit_to_celsius(temp));
        } else {
            println!("Please input a valid temperature.");
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
