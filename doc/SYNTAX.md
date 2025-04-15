# VortLang Syntax Documentation

## Comments

Comments start with `#` and continue to the end of the line:

```javascript
# This is a comment
let name = "John"; # This is also a comment
```

## Statements

Each statement does not have to be terminated with a semicolon (`;`).

```javascript
let greeting = "Hello World";
num x = 10;
print("Done");
```

Empty lines and whitespace are ignored.

## Variables

VortLang supports two types of variables:

### String Variables

String variables store text and are declared using the `let` keyword:

```javascript
let name = "John";
let greeting = "Hello, world!";
```

- String values must be enclosed in double quotes.
- String variables cannot be reassigned to numeric values.

### Numeric Variables

Numeric variables store floating-point numbers and are declared using the `num` keyword:

```javascript
num x = 5;
num y = 3.14;
num z = x plus y;
```

- Numeric variables can hold any real number.
- Numeric variables can be assigned expressions that evaluate to a number.
- Numeric variables cannot be reassigned to string values.

## Expressions

### Arithmetic Expressions

VortLang supports basic arithmetic operations:

| Symbol | Keyword      | Description |
|--------|--------------|-------------|
| `+`    | `plus`       | Addition    |
| `-`    | `minus`      | Subtraction |
| `*`    | `times`, `multiply` | Multiplication |
| `/`    | `divide`     | Division    |

Examples:
```javascript
num a = 5 plus 3;        # 8
num b = 10 minus 4;      # 6
num c = 2 times 6;       # 12
num d = 8 divide 2;      # 4
```

### Expression Evaluation

- Expressions follow standard operator precedence (multiplication/division before addition/subtraction).
- Parentheses can be used to override default precedence.
- Underscores can be used in numeric literals for readability (e.g., `1_000_000`).

Examples:
```javascript
num result1 = 2 plus 3 times 4;       # 14 (not 20)
num result2 = (2 plus 3) times 4;     # 20
num big_number = 1_000_000;           # 1000000
```

## Output

The `print` statement displays values to the console:

### Basic Print

To print a string literal:
```python
print("Hello, world!");
```

To print a variable:
```python
print(variable_name);
```

### Formatted Strings

VortLang supports string interpolation with the `o"..."` syntax:

```javascript
let name = "Alice";
num score = 95;
print(o"User {name} scored {score} points");
```

The expression inside `{}` is replaced with the corresponding variable's value.

## Error Handling

VortLang provides descriptive error messages for common issues:

### Parse Errors

- Invalid syntax: `Parse Error: Invalid variable assignment`
- Unexpected characters: `Parse Error: Unexpected character: @`
- Missing quotes: `Parse Error: String variables must be enclosed in quotes`

### Evaluation Errors

- Mismatched parentheses: `Evaluation Error: Mismatched parentheses`
- Invalid expressions: `Evaluation Error: Invalid expression`
- Unknown operators: `Evaluation Error: Unknown operator: %`

### Runtime Errors

- Undefined variables: `Runtime Error: Undefined variable: undeclared_var`
- Type mismatches: `Runtime Error: Can't change value of a string variable (name) to a number`
- Variable type errors: `Evaluation Error: Variable 'text' is a string, expected number`
