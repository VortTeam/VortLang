# Development Guide

## Architecture

- **Parser**: Converts code into tokens and evaluates expressions using the Shunting Yard algorithm.
- **Variable Storage**: Uses a `HashMap` to track variables as `VariableValue` (string or number).
