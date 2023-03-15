use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess: ");

    let mut guess = String::new(); // creates a new mutable variable currently bound to a new, empty instance of a String.

    io:: stdin()  // stdin function allows us to handle user input
        .read_line(&mut guess)   // read_line takes whatever the user types and appends that into a string
                                                          // references are immutable by default, hence we need to write &mut guess instead of &guess to make it mutable
        .expect("Failed to read line"); // read_line returns a Result enum which can be used to crash the program if the returned Result was Err

    let guess: i64 = guess.trim().parse().expect("please type a number"); // guess is originally of the type string, but since we will be comparing it to a number, we need to convert it to an integer type
                                                                              // trim removes trailing and leading spaces and newlines etc. and parses converts it to suitable datatype

    println!("You guessed: {guess}");

    println!("You guessed: {}", guess); // gives the same output as the above line

    println!("The secret number was {secret_number}");

    match guess.cmp(&secret_number){ // cmp compares the two numbers and returns a variant of Ordering
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }

    // loop{
    //     println!("Please input your guess: ");
    //             -- snip for taking input in guess and generating secret_number --     
    //      -- snip for matching guess with secret_number --
    // }

    loop {
        println!("\n\nGuess the number!\n");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line( &mut guess)
            .expect("Unable to read line");

        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
    
}
