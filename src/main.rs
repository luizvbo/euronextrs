use clap::{Arg, Command};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the command-line arguments
    let matches = Command::new("Euronext Data Extractor")
        .version("1.0")
        .author("Luiz Otavio V. B. Oliveira <luiz.vbo@gmail.com>")
        .about("Extracts financial data from the Euronext website")
        .arg(
            Arg::new("isin")
                .short('i')
                .long("isin")
                .value_name("ISIN")
                .help("The ISIN of the instrument to fetch data for")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format string: use %p for price, %o for since open, %O for since open percentage, %c for since close, %C for since close percentage")
                .default_value("%p - Open: %o (%O) - Close: %c (%C)")
                .num_args(1),
        )
        .get_matches();

    // Get the ISIN and format from arguments
    let isin = matches.get_one::<String>("isin").unwrap();
    let format = matches.get_one::<String>("format").unwrap();

    // Construct the URL
    let url = format!(
        "https://live.euronext.com/en/ajax/getDetailedQuote/{}-XAMS",
        isin
    );

    // Fetch the HTML from the URL
    let client = Client::new();
    let response = client.get(&url).send()?.text()?;

    // Parse the HTML
    let document = Html::parse_document(&response);

    // Define CSS selectors
    let price_selector = Selector::parse("#header-instrument-price").unwrap();
    let since_open_selector =
        Selector::parse(".data-header__col-right .col:nth-of-type(1) span.data-24").unwrap();
    let since_close_selector =
        Selector::parse(".data-header__col-right .col:nth-of-type(2) span.data-24").unwrap();

    // Extract data
    let price = document
        .select(&price_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("0")
        .replace(",", "") // If the price uses commas as a thousands separator, remove them
        .parse::<f64>()
        .unwrap_or(0.0);

    let since_open = document
        .select(&since_open_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("0")
        .replace(",", "") // If the price uses commas as a thousands separator, remove them
        .parse::<f64>()
        .unwrap_or(0.0);

    let since_close = document
        .select(&since_close_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("0")
        .replace(",", "") // If the price uses commas as a thousands separator, remove them
        .parse::<f64>()
        .unwrap_or(0.0);

    let since_open_percent = since_open / price;
    let since_close_percent = since_open / price;

    // Format the output
    let output = format
        .replace("%p", &price.to_string())
        .replace("%o", &since_open.to_string())
        .replace("%O", &format!("{:.2}%", since_open_percent * 100.0))
        .replace("%c", &since_close.to_string())
        .replace("%C", &format!("{:.2}%", since_close_percent * 100.0));
    println!("{}", output);
    Ok(())
}
