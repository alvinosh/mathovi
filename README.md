# Mathovi

Mathovi is a simple project inteded to make generating math equations easier.

It works by taking in a series of semicolon seperated math equations and producing a png of these mathematical equations using LaTeX.

Currently there is only support for simple algebra and simple trigonometric functions such as sin and cos, however I plan on adding more complicatied notation such as derivatives and integrals

## Dependencies

You will need LaTeX installed on your machine.

## Quick Start

```console
cargo build --release
mathovi --input <input_file> --output <output_file>
```

## Example

```
a = 5 / 5 + 5;

b = sqrt(5);

a = (sin(5 * x) - 5) / 3;
```

![example](example/basic.png)

## Plans

- Support For Simple Calculus Expressions, derivatives / integrals
- Support For Linear Algebra Expressions, Vectors / Matrices
