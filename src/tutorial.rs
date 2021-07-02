use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("Guess the 番号!");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_num);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}
