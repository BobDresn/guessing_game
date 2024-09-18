use std::io;
use rand::Rng;

fn main() {
    //Intro
    println!("Welcome to the guessing game!");

    //Only needs to be created when the game starts
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts: u8 = 0;

    let mut game_over = false;

    //Game loop
    while !game_over {
        println!("Enter your guess between 1 and 100: ");

        //Takes input as guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Checks input is a number
        let guess: u8 = match guess.trim().parse().expect("Please type a number!");
        /**
        USE MATCH CASE TO SPECIFY ERROR HANDLING, Err(_) CAN BE SET TO ANY ANSWER FOR FAILURE
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
        */

        //Checks if guess is correct
        if guess > secret_number {
            println!("Your guess was too high!");
            attempts += 1;
        } else if guess < secret_number {
            println!("Your guess was too low!");
            attempts += 1;
        } else {
            attempts += 1;
            println!("Your guess was correct! You won!");
            println!("It took you {} attempts!", attempts);
            game_over = true;
        }

        //Another way to handle 
        /**
        attempts += 1;
        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!");
        println!("It took {attempts} attempts"),
        }
        */
    }
}
