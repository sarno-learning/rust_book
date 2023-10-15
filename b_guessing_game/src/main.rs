use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to my guessing game!\n");

    'main: loop {
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        println!("Please input your guess.");
        loop {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("\nPlease input a valid number.");
                    continue;
                },
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("\nNope, too small! try again."),
                Ordering::Greater => println!("\nNope, too large! try again."),
                Ordering::Equal => {
                    println!("\nYes, you win! Try again? (yes/no)");

                    let mut message = String::new();
                    io::stdin()
                        .read_line(&mut message)
                        .expect("Failed to read line.");

                    let message = message.to_lowercase();
                    if message.contains("yes") {
                        println!("");
                        break;
                    }

                    break 'main;
                },
            }
        }
    }
}
