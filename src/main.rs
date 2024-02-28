use rand::Rng;
use std::{io, vec};

fn guess_game() {
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

fn exercise_1() {
    println!("Entering exercise_1: Sequence");
    let vector = vec![1, 2, 3];
    // loop over the vector with enumeration
    for (i, item) in vector.iter().enumerate() {
        println!("{i}th item is {item}");
    }

    println!("Entering exercise_1: Hashmap");
    let mut map = std::collections::HashMap::new();
    map.insert("1", "Hi");
    map.insert("Hello", "0");

    for (key, value) in map.iter() {
        println!("{key} is {value}");
    }
}

fn exercise_2() {
    println!("Entering exercise_2");
    return;
}

fn exercise_3() {
    println!("Entering exercise_3");
    return;
}
fn exercise_4() {
    println!("Entering exercise_4");
    return;
}

fn main() {
    let mut func_count = String::new();
    println!("Enter the program to execute");
    io::stdin()
        .read_line(&mut func_count)
        .expect("Cannot read input");

    let func_count_int = func_count
        .trim()
        .parse()
        .expect("Cannot parse input to int");
    match func_count_int {
        0 => guess_game(),
        1 => exercise_1(),
        2 => exercise_2(),
        3 => exercise_3(),
        4 => exercise_4(),
        _ => println!("Invalid exercise number"),
    }
}
