# Timewave Zero Calculator

This Rust application calculates Timewave Zero values based on the theories of Terence McKenna. It implements four different datasets: Kelley, Watkins, Sheliak, and Huang Ti.

## Features

- Calculate Timewave Zero values for a specified time range
- Support for four different datasets: Kelley, Watkins, Sheliak, and Huang Ti
- Customizable wave factor
- Optional multi-threading for improved performance on multi-core systems

## Installation

1. Ensure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
    git clone https://github.com/literally-sai/timewave_zero.git
    cd tunewave_zero
3. Build the project:
    cargo build --release

## Usage

Run the program with the following command-line arguments:
    cargo run --release -- --dtz <DAYS_TO_ZERO> --negative-bailout <DAYS_AFTER_ZERO> --step <MINUTES> [--wave-factor <FACTOR>]

- `--dtz`: Number of days before the zero point to start calculations
- `--negative-bailout`: Number of days after the zero point to end calculations
- `--step`: Time step in minutes
- `--wave-factor`: Wave factor (default is 64)

Example:
cargo run --release -- --dtz 365 --negative-bailout 30 --step 60 --wave-factor 72

This calculates the timewave for one year before the zero point to 30 days after, in 1-hour steps, with a wave factor of 72.

## Multi-threading

To enable multi-threading, use the `threaded` feature:

cargo run --release --features threaded -- <ARGS>

## Docker

To run the application using Docker:

1. Build the Docker image:

docker build -t timewave-zero .

2. Run the container:

docker run --rm timewave-zero --dtz 365 --negative-bailout 30 --step 60 --wave-factor 72

Replace the arguments as needed.

## Output

The program outputs CSV data to stdout with the following columns:

DTZ, Kelley, Watkins, Sheliak, Huang Ti

Where:
- `DTZ`: Days to Zero
- `Kelley`, `Watkins`, `Sheliak`, `Huang Ti`: Timewave values for each dataset

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgements

This project is based on the Timewave Zero theory by Terence McKenna and uses datasets developed by various researchers in the field.

## Disclaimer

This software is provided for educational and entertainment purposes only. The Timewave Zero theory is not scientifically validated and got disproven multiple times.