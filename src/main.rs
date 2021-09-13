use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("***** Welcome to Guess the Number! *****");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut is_first_try = true;

    loop {
        let mut guess = String::new();

        if is_first_try {
            println!("Please input your guess: ");
            is_first_try = false;
        } else {
            println!("Try again: ");
        }

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number...");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("***** The end. *****")
}
