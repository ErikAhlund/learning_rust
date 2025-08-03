use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");
    println!("Guess a number between 0 and 100");
    let number = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(_) => {
                println!("Something went wrong reading your input");
                continue;
            }
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a positive number!");
                continue;
            }
        };

        println!("Your guess was {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
        }
    }
}
