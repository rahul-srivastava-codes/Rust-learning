use std::io;

//number guessing game
// fn main() {
//     println!("Number Guessing Game");
//     println!("Please input your guess.");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
//     let x = 4;
//     println!("{x}");
// }

//Generating a random number
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed it right");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal =>
        /*block:*/
        {
            println!("You win");
            // break 'block;
        }
    }
}
