use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start() {
    println!("Guess the 番号!");
    let secret_num = generate_secret_num();
    println!("The secret number is: {}", secret_num);
    println!("Please input your guess.");
    let guess = input_guess();
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn generate_secret_num() -> i32 {
    let secret_num = rand::thread_rng().gen_range(1..101);
    secret_num
}

fn input_guess() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().expect("Please type a number!")
}
