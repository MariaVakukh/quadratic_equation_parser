use anyhow::{Result, anyhow};
use pest::Parser;
use quadratic_equation_parser::*;
use std::io::{self, Write};
use std::{env, fs};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Err(anyhow!("no command provided, use 'help' for help idk..."));
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                return Err(anyhow!("no file name provided"));
            }
            parse_file(&args[2])?;
        }
        "input" => interactive_mode()?,
        "help" => print_help(),
        "credits" => print_credits(),
        cmd => {
            return Err(anyhow!(
                "i dont know this command: '{}'. use 'help' for available commands.",
                cmd
            ));
        }
    }

    Ok(())
}

fn interactive_mode() -> Result<()> {
    println!("**** Interactive mode. Enter 'exit' to quit ****");

    loop {
        print!("enter a !quadratic! equation: ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let line = line.trim();

        if line.eq_ignore_ascii_case("exit") {
            break;
        }
        if line.is_empty() {
            continue;
        }

        match Grammar::parse(Rule::equation, line) {
            Ok(pairs) => match extract_coeff(pairs) {
                Ok((a, b, c)) => {
                    println!("Coefficients: a={}, b={}, c={}", a, b, c);
                    match solve(a, b, c) {
                        Some((x1, x2)) => println!("Roots: x1 = {}, x2 = {}", x1, x2),
                        None => {
                            if b != 0.0 {
                                println!("Linear root: x = {}", -c / b);
                            } else {
                                println!("No real roots");
                            }
                        }
                    }
                }
                Err(e) => println!("Error extracting coefficients: {}", e),
            },
            Err(e) => println!("Error parsing equation: {}", e),
        }

        println!("-------------------------");
    }

    Ok(())
}
fn parse_file(file_name: &str) -> Result<()> {
    let content = fs::read_to_string(file_name)?;

    for line in content.lines().filter(|l| !l.trim().is_empty()) {
        let pairs = Grammar::parse(Rule::equation, line)?;
        let (a, b, c) = extract_coeff(pairs)?;

        println!("Equation: {}", line);
        println!("Coefficients: a={}, b={}, c={}", a, b, c);

        match solve(a, b, c) {
            Some((x1, x2)) => println!("Roots: x1 = {}, x2 = {}", x1, x2),
            None => {
                if b != 0.0 {
                    println!("Linear root: x = {}", -c / b);
                } else {
                    println!("No roots");
                }
            }
        }
        println!("-------------------------");
    }

    Ok(())
}

pub fn print_help() {
    let help_text = r#"
==================== Quadratic Equation Parser ====================

How to use:
    cargo run -- <command>

Commands:
    parse <file_name>     parses the file and calculates roots of equations
    input                 Enter equations directly in the console
    help                  displays this help message
    credits               shows credits 

Example for input:
    2*x^2 - 3*x + 1 = 0
    x^2+5*x-6=0

=======================================================================
"#;
    println!("{}", help_text);
}

pub fn print_credits() {
    let credits = r#"
====================== Quadratic Equation Parser ======================

The parser for quadratic equations that extracts coefficients and computes real roots.
Developed by Maria Vakukh. 
(^•-•^)

=======================================================================
"#;
    println!("{}", credits);
}
