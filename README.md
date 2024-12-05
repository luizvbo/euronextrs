# Euronext-rs

Euronext-rs is a command-line application written in Rust. It fetches real-time
trading data from the Euronext website and outputs it in a customizable format.
This can be particularly useful for financial analysis, algorithmic trading, and other
applications that require real-time stock market data.

## Repository

The source code for Euronext-rs is hosted on GitHub. You can clone or fork the
repository from the following URL:

[https://github.com/luizvbo/euronextrs](https://github.com/luizvbo/euronextrs)

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

The executable will be located in the target/release directory.

## Usage

You can run Euronext-rs from the terminal with the following command:

```bash
./target/release/euronextrs <ISIN> [<FORMAT>]
```

### Arguments

- **ISIN**: The ISIN of the instrument to fetch data for. This is required and should follow the format `{ISIN}-XAMS` (e.g., `IE00B4L5Y983-XAMS`).

- **FORMAT** (optional): A custom output format string that defines how the data is displayed. The default format is:
`"%p | Open: %o (%O) | Close: %c (%C)"`

You can use the following placeholders in the format string:

- `%p`: The price.
- `%o`: The value since open.
- `%O`: The percentage change since open.
- `%c`: The value since close.
- `%C`: The percentage change since close.

For example, to customize the output to show only the price and open percentage, you can use the following format:

```bash
./target/release/euronextrs IE00B4L5Y983-XAMS "%p - %O"
```

### Example

```bash
./target/release/euronextrs IE00B4L5Y983-XAMS
```

This will fetch the data for the instrument with the specified ISIN and print
it in the default format.

For example, the output could look like:

```
123.45 | Open: 120.00 (2.5%) | Close: 121.00 (1.9%)
```

#### Custom Format Example
```bash
./target/release/euronextrs IE00B4L5Y983-XAMS "%p | Open: %o | Change: %O"
```

This will print something like:

```
123.45 | Open: 120.00 | Change: 2.5%
```

## Contributing
Contributions to Euronext-rs are welcome! Please feel free to open an issue or submit a pull request on GitHub.


## Key Changes:
- **Usage Instructions**: Added description of the new `FORMAT` argument that allows users to specify custom output formatting.
- **Example Format Strings**: Included example format strings to show how users can control the output.
- **Output Description**: Updated output example to reflect the new features and formatting options.
