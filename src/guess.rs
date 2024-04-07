use rand::Rng;
use std::io;

pub fn guess_game() {
    println!("Entering guess_game");

    let secret_num = rand::thread_rng().gen_range(0..=100);
    let mut success: bool = false;
    let mut counter: i32 = 0;
    while success != true {
        let mut guess = String::new();
        println!("Enter your guess!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string");
        // cast int to string
        let int_guess: i32 = guess.trim().parse().expect("Cannot parse guess to int");
        if int_guess == secret_num {
            println!("Congrats you are correct after {counter} attempts, the secret number is {secret_num}");
            success = true;
        } else {
            if int_guess > secret_num {
                println!("Your guess is high");
            } else {
                println!("Your guess is low");
            }
            println!("You are wrong, try again");
        }
        counter = counter + 1;
    }
}
fn main() {
    guess_game();
}
