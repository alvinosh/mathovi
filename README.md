# Mathovi

The goal of this project is to make it easier to quickly write math equations.

## Dependencies

You need LaTeX installed on your machine

## Usage

```console
cargo build --release
mathovi --input <input_file> --output <output_file>
```

## Syntax Example

```
a = (sqrt(5 ^ (23 + 5)) / 3) * 21
```

## Supported Functions

- Sqrt
- Sin
- Cos

## Plans

- Support For Simple Calculus Expressions, derivatives / integrals
- Support For Linear Algebra Expressions, Vectors / Matrices
