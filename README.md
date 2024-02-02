# Euronext-rs

Euronext-rs is a command-line application written in Rust. It fetches real-time
trading data from the Euronext API and writes it to a CSV file. This can be
particularly useful for financial analysis, algorithmic trading, and other
applications that require real-time stock market data.

## Repository

The source code for Euronext-rs is hosted on GitHub. You can clone or fork the
repository from the following URL:

https://github.com/luizvbo/euronextrs

## Installation

To install Euronext-rs, you need to have Rust installed on your machine. If you
don't have Rust installed, you can download it from the official website.

Once you have Rust installed, you can clone the Euronext-rs repository and build
the application using Cargo, Rust's package manager. Here are the steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/luizvbo/euronextrs.git
   ```

2. Navigate into the cloned repository:

   ```bash
   cd euronextrs
   ```

3. Build the application:

   ```bash
   cargo build --release
   ```

The executable will be located in the `target/release` directory.

## Usage

You can run Euronext-rs from the terminal with the following command:

```bash
./target/release/euronextrs <OUTPUT_PATH> <ISIN>
```

Replace <OUTPUT_PATH> with the path where you want the CSV file to be written,
and <ISIN> with the ISIN and stock exchange code (as given in the Euronext URL).
The default ISIN is "IE00B4L5Y983-XAMS".

For example:

```bash
./target/release/euronextrs output.csv IE00B4L5Y983-XAMS
```

This will fetch data from the Euronext API and write it to output.csv.

## Contributing

Contributions to Euronext-rs are welcome! Please feel free to open an issue or
submit a pull request on GitHub.
