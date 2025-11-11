# Description

This project implements a parser for quadratic equations using the Pest.
The parser recognizes and validates equations of the form:

ax^2 + bx + c = 0

Each equation is parsed into a coeficients for further mathematical processing.

In parsing process it tokenizes input into numbers, terms (x², x), signs, and operators wnd then extracts coefficients from parsed terms.
This coefficients are used to calculate roots of equation. 

This project functions as a full-featured command-line application.

## Available Commands

| Command | Description | 
|---------|-------------|
| `cargo run -- input`| Interactive mode for your equation input |
| `cargo run -- parse equations.txt` | Batch process equations from a text file |  
| `cargo run -- help` | Display usage instructions and examples |  
| `cargo run -- credits` | Show project information and author |  

And also it includes a Makefile with convenient commands for development and testing.

## Make Commands

| Command | Description |
|---------|-------------|
| `make run` | Run the program with equation.txt file |
| `make interactive` | Run in interactive mode |
| `make test` | Run all tests |
| `make fmt` | Format the code using rustfmt |
| `make clippy` | Run clippy for code linting |
| `make precommit` | Run format + clippy + tests (pre-commit check) |
| `make build` | Build release version |
| `make help` | Show all available make commands |


# How to Run

1. Clone and build this project:
  ```
   git clone <repository_url>
   cd quadratic_equation_parser
   cargo build
  ```
2. Open the project folder in your terminal.
3. Run one of the available commands (use `cargo run -- help` if you lost).
4. To run tests:
   cargo test

# Example

**Input:**
```
x^2 - 5x + 6 = 0
2x^2 + 7x + 3 = 0
```

**Output:**
```
Equation: x^2 - 5x + 6 = 0
Coefficients: a=1, b=-5, c=6
Roots: x=2, x=3

Equation: 2x^2 + 7x + 3 = 0
Coefficients: a=2, b=7, c=3
Roots: x=-0.5, x=-3
```


