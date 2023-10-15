use std::io;

fn main() {
    println!("Welcome to my fibonacci generator!");

    loop {
        println!("\nEnter the number you would like to generate to, or q to quit.");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n = n.trim().to_lowercase();
        if n.contains('q') {
            break;
        }

        let n: u64 = match n.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        fibonacci(n);
    }
}

fn fibonacci(n: u64) {  
    let mut f1: u64 = 0;
    let mut f2: u64 = 1;
    for index in 0..n+1 {
        match index {            
            0 => {
                println!("\n{index}: {f1}");
            },
            1 => {
                println!("{index}: {f2}");
            },
            2.. => {
                let new_value = f1 + f2;
                f1 = f2;
                f2 = new_value;
        
                println!("{index}: {new_value}");
            },
        }
    }
}
