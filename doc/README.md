# Vortlang Documentation

## Introduction

Vortlang is a simple programming language designed for clarity and ease of use. The compiler translates Vortlang code into C, which is then compiled into an executable. This documentation covers the language features, syntax, and provides examples of Vortlang programs.

## Language Features

Vortlang supports:
- String and numeric variables
- Mathematical expressions with proper operator precedence
- Print statements with optional string formatting
- Comments

## Syntax

### Variables

Vortlang has two types of variables:
- String variables: Declared with `let`
- Numeric variables: Declared with `num`

```rust
let message = "Hello, World!"
num value = 42
```

Variables can be reassigned after declaration:

```rust
message = "New message"
value = 100
```

### Print Statements

To print values:
- Basic print: `print("Hello")`
- Variable print: `print(message)`
- Formatted print: `print(o"Value: {value}")`

### Mathematical Expressions

Vortlang supports basic arithmetic operations:
- Addition: `+` or `plus`
- Subtraction: `-` or `minus`
- Multiplication: `*` or `times` or `multiply`
- Division: `/` or `divide`

Expressions follow standard operator precedence and can be grouped with parentheses.

### Comments

Single-line comments start with `//`:

```rust
// This is a comment
```

## Example Programs


```rust
// A simple hello world program

// Print a greeting to the console
print("Hello, World!")

// Using a variable
let message = "Welcome to Vortlang!"
print(message)

```

```rust
// Demonstrating numeric operations

// Declare numeric variables
num x = 10
num y = 5

// Basic operations
num sum = x + y
num difference = x - y
num product = x * y
num quotient = x / y

// Print results
print(o"Sum: {sum}")
print(o"Difference: {difference}")
print(o"Product: {product}")
print(o"Quotient: {quotient}")

// Complex expression with precedence
num result = (x + y) * (x - y)
print(o"(x + y) * (x - y) = {result}")

// Using word operators
num a = 3
num b = 2
num c = a plus b multiply 4 divide 2 minus 1
print(o"3 plus 2 multiply 4 divide 2 minus 1 = {c}")

```

```rust
// Demonstrating string variables and formatting

// String variables
let name = "Alice"
let greeting = "Hello"

// Basic string printing
print(name)
print(greeting)

// String formatting with variables
print(o"{greeting}, {name}!")

// Mixing string and numeric variables
num age = 25
print(o"{name} is {age} years old.")

// Multiple variables in one format string
num height = 5.7
print(o"{name} is {age} years old and {height} feet tall.")

```

```rust
// Demonstrating variable reassignment

// Initial values
let name = "Bob"
num count = 1

// Print initial values
print(o"Name: {name}")
print(o"Count: {count}")

// Reassign variables
name = "Alice"
count = count + 1

// Print new values
print(o"Updated name: {name}")
print(o"Updated count: {count}")

// Multiple reassignments
count = count * 2
count = count + 5
print(o"Final count: {count}")

```

```rust
// Convert between Celsius and Fahrenheit

// Initial temperature in Celsius
num celsius = 25

// Convert to Fahrenheit: F = C * 9/5 + 32
num fahrenheit = celsius * 9 / 5 + 32

// Print the result
print(o"Temperature conversion:")
print(o"{celsius} degrees Celsius equals {fahrenheit} degrees Fahrenheit")

// Convert back to Celsius: C = (F - 32) * 5/9
num celsius_check = (fahrenheit - 32) * 5 / 9
print(o"Converted back: {celsius_check} degrees Celsius")

```

```rust
// Calculate properties of a circle

// Define Pi as a constant
num PI = 3.14159

// Define the radius of the circle
num radius = 5

// Calculate area: A = π * r²
num area = PI * radius * radius

// Calculate circumference: C = 2 * π * r
num circumference = 2 * PI * radius

// Calculate diameter: d = 2 * r
num diameter = 2 * radius

// Print results
print(o"Circle properties (radius = {radius}):")
print(o"Diameter: {diameter}")
print(o"Circumference: {circumference}")
print(o"Area: {area}")

```

```rust
// Simple investment calculator

// Initial investment amount
num principal = 1000

// Annual interest rate (as decimal)
num rate = 0.05

// Number of years
num years = 10

// Simple interest calculation: I = P * r * t
num simple_interest = principal * rate * years
num simple_total = principal + simple_interest

// Print simple interest results
print(o"Investment Calculator:")
print(o"Principal: {principal}")
print(o"Annual rate: {rate}")
print(o"Time period: {years} years")
print(o"Simple interest: {simple_interest}")
print(o"Final amount with simple interest: {simple_total}")

// Compound interest for first year (manual calculation)
num year1 = principal * (1 + rate)
num year2 = year1 * (1 + rate)
num year3 = year2 * (1 + rate)

print(o"Compound interest after 1 year: {year1}")
print(o"Compound interest after 2 years: {year2}")
print(o"Compound interest after 3 years: {year3}")

```

## Compiler Usage

To compile a Vortlang program:

```
cargo run --release <path/to/filename.vl>
```

This will compile the program and create an executable with the same name (`filename.exe`).

## Error Handling

The Vortlang compiler provides helpful error messages that include:
- The location of the error (line and column)
- A description of the error
- A hint for fixing the error

For example, if you try to use a variable before declaring it:

```
print(a)  // Error: Variable 'a' used before declaration
```

The compiler will show where the error occurred and suggest declaring the variable first.

## Limitations

- Vortlang does not support functions, loops, or conditional statements
- There are only two data types: strings and numbers (represented as double-precision floating-point)
- No arrays or data structures
- No file I/O operations

## Best Practices

1. Use descriptive variable names
2. Add comments to explain your code
3. Use format strings (`print(o"...")`) for clearer output
4. Use parentheses to make complex expressions more readable

These examples demonstrate the core features of Vortlang, from basic printing to more complex arithmetic operations and string formatting. Each example can be compiled and run using the Vortlang compiler.
