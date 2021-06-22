/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the 番号!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_num);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
