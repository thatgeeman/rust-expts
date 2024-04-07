use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("Entering guess_game (cmp)");
    let secret_num = rand::thread_rng().gen_range(0..=100);
    loop {
        println!("Enter your guess [1, 100]!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string");
        // cast int to string
        // can use same name due to shadowing
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("That was not a number, continue..\n");
                continue;
            }
        };
        // parse returns error type Ok or Err

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
fn main() {
    guess_game();
}
