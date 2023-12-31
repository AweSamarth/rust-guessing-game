use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nNumber guessing game 🤓☝️");
    println!("Type 'exit' to exit the program\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess: ");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");

        match guess.as_str().trim() {
            "exit" => {
                println!("Bye!");
                break;
            }

            _ => {}
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed it right! Congratulations");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
        }
    }
}
