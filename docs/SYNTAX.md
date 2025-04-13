# VortLang Syntax Documentation

## Variables

### String Variables
- Declare with `let`.
- Must be enclosed in double quotes.
```python
let greeting = "Hello"
```

### Numeric Variables
- Declare with `num`.
- Supports arithmetic expressions.
```python
num value = 5 plus (3 times 2)  # 11
```

## Print Statements

### Syntax
```python
print(<content>)
```

### Examples
1. Print a literal string:
   ```python
   print("Hello World")
   ```
2. Print a variable:
   ```python
   print(x)
   ```
3. Formatted string (with `o"..."`):
   ```python
   print(o"{greeting} User! Score: {score}")
   ```

## Expressions

### Valid Operations
- Use parentheses `()` for precedence.
- Operator keywords are replaced internally (e.g., `plus` becomes `+`).

Example:
```python
num result = (10 minus 2) divide 4  # Evaluates to 2
```

## Error Handling

Common errors include:
- Undefined variables: `Error: Undefined variable: x`
- Invalid assignments: `Error: String variables must be enclosed in quotes`
- Mismatched parentheses: `Error: Mismatched parentheses`
