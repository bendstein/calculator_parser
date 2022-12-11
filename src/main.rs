use std::io::Write;

#[macro_use]
extern crate lazy_static;

pub mod calculator_parser;
pub mod calculator_interpreter;

const EXIT_COMMAND: &str = ":exit";
const CLEAR_COMMAND: &str = ":clear";

fn main() {
    //Set to use virtual terminal so that control characters work on windows
    _ = colored::control::set_virtual_terminal(true);

    println!("Enter the expression to evaluate, '{CLEAR_COMMAND}' to clear the screen, or '{EXIT_COMMAND}' to exit.");

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
            break;
        }
        else if input.eq_ignore_ascii_case(CLEAR_COMMAND) {
            print!("{esc}c", esc = 27 as char);
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

        let evaluated = match calculator_interpreter::Interpreter::default().evaluate(parsed) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("An error occurred while evaluating expression '{input}': {e}");
                continue;
            }
        };

        println!("{evaluated}");
    };
}