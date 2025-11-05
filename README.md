# Description

This project implements a parser for quadratic equations using the Pest.
The parser recognizes and validates equations of the form:

ax^2 + bx + c = 0

Each equation is parsed into a coeficients for further mathematical processing.

In parsing process it tokenizes input into numbers, terms (x², x), signs, and operators wnd then extracts coefficients from parsed terms.
This coefficients are used to calculate roots of equation.

# Example

**Input:**

x^2 - 5x + 6 = 0
2x^2 + 7x + 3 = 0


**Output:**

Equation: x^2 - 5x + 6 = 0
Coefficients: a=1, b=-5, c=6
Roots: x=2, x=3

Equation: 2x^2 + 7x + 3 = 0
Coefficients: a=2, b=7, c=3
Roots: x=-0.5, x=-3



