use std::cmp::Ordering;

mod generate;
mod input;

pub fn guess_game() {
    println!("Guess the 番号!");
    let secret_num = generate::generate_secret_num();
    println!("The secret number is: {}", secret_num);
    println!("Please input your guess.");
    let guess = input::input_guess();
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}
