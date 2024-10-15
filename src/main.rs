use codecrafters_interpreter::lox::Lox;
use std::io::BufRead;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file = fs::read_to_string(filename).unwrap();
            let lox = Lox::new(&file);
            for token in &lox.tokens {
                println!("{}", token);
            }
            if lox.had_error() {
                std::process::exit(65);
            }
        },
        "parse" => {
            let file = fs::read_to_string(filename).unwrap();
            let lox = Lox::new(&file);
            let result = lox.run().unwrap();
            if lox.had_error() {
                std::process::exit(65);
            }
            println!("{}", result);
        },
        "run" => {
            let file = fs::read_to_string(filename).unwrap();
            let lox = Lox::new(&file);
            let result = lox.run().unwrap();
            if lox.had_error() {
                std::process::exit(65);
            }
            println!("< {}", result);
        },
        "run-prompt" => loop {
            println!(">");
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut input = String::new();
            handle.read_line(&mut input).unwrap();
            if input == *"" {
                break;
            }
            let lox = Lox::new(&input);
            let result = lox.run().unwrap();
            println!("< {}", result);
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        },
    }
}
