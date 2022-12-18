#[macro_use]
extern crate lazy_static;

pub mod calculator_logic;
pub mod calculator_interface;

use std::io::Write;
use calculator_logic::{calculator_interpreter, calculator_parser};

const EXIT_COMMAND: &str = ":exit";
const CLEAR_COMMAND: &str = ":clear";
const CLEAR_HISTORY_COMMAND: &str = ":clear-hist";
const CLEAR_MEMORY_COMMAND: &str = ":clear-mem";

fn main() {
    //Set to use virtual terminal so that control characters work on windows
    _ = colored::control::set_virtual_terminal(true);

    let mut interpreter = calculator_interpreter::interpreter::Interpreter::default();

    println!("Enter the expression to evaluate, '{CLEAR_COMMAND}' to clear the screen, '{CLEAR_HISTORY_COMMAND}' to clear result history, '{CLEAR_MEMORY_COMMAND}' to clear calculator memory, or '{EXIT_COMMAND}' to exit.");

    loop {
        print!("> ");
        
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to flush stdout: {err}");
                continue;
            }
        }

        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to read input: {err}");
                continue;
            }
        }

        input = String::from(input.trim_end());

        if input.eq_ignore_ascii_case(EXIT_COMMAND) {
            println!("Exiting...");
            break;
        }
        else if input.eq_ignore_ascii_case(CLEAR_COMMAND) {
            print!("{esc}c", esc = 27 as char);
            continue;
        }
        else if input.eq_ignore_ascii_case(CLEAR_HISTORY_COMMAND) {
            interpreter.clear_stack();
            println!("Cleared calculator history.");
            continue;
        }
        else if input.eq_ignore_ascii_case(CLEAR_MEMORY_COMMAND) {
            interpreter.clear_mem();
            println!("Cleared calculator memory.");
            continue;
        }

        let mut parser = calculator_parser::parser::Parser::new(&input);

        let parsed = match parser.parse() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("An error occurred while parsing expression '{input}'. At {}: {e}", parser.lah());
                continue;
            }
        };

        let evaluated = match interpreter.evaluate(parsed) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("An error occurred while evaluating expression '{input}': {e}");
                continue;
            }
        };

        println!("{evaluated}");
    };
}