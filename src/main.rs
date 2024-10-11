mod lexer;
mod reporter;
mod scanner;

use crate::lexer::Token;
use crate::reporter::Reporter;
use crate::scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, BufRead};

struct Lox {
    reporter: Reporter,
    tokens: Vec<Token>,
}

impl Lox {
    pub fn new(source: &str) -> Self {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        Self { reporter: scanner.reporter, tokens: scanner.tokens }
    }

    pub fn run(&self) -> String {
        let tokens: Vec<String> = self.tokens.iter().map(|t| t.to_string()).collect();
        tokens.join(" ")
    }

    pub fn had_error(&self) -> bool {
        !self.reporter.errors.is_empty()
    }
}

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
            let file = read_file(filename);
            let lox = Lox::new(&file);
            for token in &lox.tokens {
                println!("{}", token);
            }
            if lox.had_error() {
                std::process::exit(65);
            }
        },
        "run" => {
            let file = read_file(filename);
            let lox = Lox::new(&file);
            let result = lox.run();
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
            let result = lox.run();
            println!("< {}", result);
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        },
    }
}

fn read_file(filename: &String) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    })
}
