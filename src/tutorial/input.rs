use std::io;

pub fn input_guess() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().expect("Please type a number!")
}
