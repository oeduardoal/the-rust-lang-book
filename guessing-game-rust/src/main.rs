use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries = 0;

    loop {
        let mut guess = String::new();

        println!("Please input your guess");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        tries += 1;

        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please type a number! The {} is not a number!", err.clone().to_string());
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your guess is too small".red()),
            Ordering::Greater => println!("{}", "Your guess is too big".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("You guessed {} times!", tries);
                break;
            },
        }

    }


}
