# Temperature Conversion CLI

A Rust-based command-line application for converting temperatures between Celsius, Fahrenheit, and Kelvin.

## Features

- Convert Celsius to Fahrenheit
- Convert Celsius to Kelvin
- Convert Fahrenheit to Celsius
- Convert Fahrenheit to Kelvin
- Convert Kelvin to Celsius
- Convert Kelvin to Fahrenheit

## Project Structure

src/
├── main.rs
├── lib.rs
├── cli/
│ ├── mod.rs
│ └── get_user_input.rs
└── models/
├── mod.rs
├── temperature.rs
└── temperature_conversion.rs

- `main.rs`: The entry point of the application.
- `lib.rs`: Contains library code and module declarations.
- `cli/`: Contains the command-line interface logic.
- `models/`: Contains the temperature models and conversion logic.

## Installation

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```sh
   git clone https://github.com/Deepakolee785/rust_temperate_converter.git
   cd rust_temperate_converter
   ```

## Usage

Run the application:
`cargo run`

## Usage/Examples

```bash
Please Choose an Option

1. Celsius to Fahrenheit
2. Celsius to Kelvin
3. Fahrenheit to Celsius
4. Fahrenheit to Kelvin
5. Kelvin to Celsius
6. Kelvin to Fahrenheit
7. Exit

Enter your choice:
1
Enter the temperature value:
37
98.60°F

Please Choose an Option

1. Celsius to Fahrenheit
2. Celsius to Kelvin
3. Fahrenheit to Celsius
4. Fahrenheit to Kelvin
5. Kelvin to Celsius
6. Kelvin to Fahrenheit
7. Exit

Enter your choice:

```
