use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Print the game name
    println!("Guess the number!");

    // Generate the secret number to be guessed by the user
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Begin a loop
    loop {
        println!("Please input your guess.");

        // Initiate the guess variable
        let mut guess = String::new();

        // Read user input and handle error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Transform the user input into a numeric value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the number the user guessed
        println!("You guessed: {guess}");

        // Test all possible cases
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Break the loop when the user guesses it right!
                break;
            }
        }
    }
}
