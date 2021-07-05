/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
use std::env;
use std::process;

mod tutorial;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        process::exit(1);
    }

    match args[1].as_str() {
        "guess" => tutorial::guess_game::start(),
        "ownership" => tutorial::ownership::start(),
        "structure" => tutorial::structure::start(),
        _ => process::exit(2),
    }
}
