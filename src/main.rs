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
                .help("Output format: price, abs, rel")
                .num_args(1)
                .required(true)
                .value_parser(["price", "abs", "rel"]),
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
    let since_open_selector = Selector::parse(".data-header__col-right .col:nth-of-type(1) span.data-24").unwrap();
    let since_open_percent_selector = Selector::parse(".data-header__col-right .col:nth-of-type(1) span.text-ui-grey-1").unwrap();
    let since_previous_selector = Selector::parse(".data-header__col-right .col:nth-of-type(2) span.data-24").unwrap();
    let since_previous_percent_selector = Selector::parse(".data-header__col-right .col:nth-of-type(2) span.text-ui-grey-1").unwrap();

    // Extract data
    let price = document
        .select(&price_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("N/A");

    let since_open = document
        .select(&since_open_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("N/A");

    let since_open_percent = document
        .select(&since_open_percent_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("N/A");

    let since_previous = document
        .select(&since_previous_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("N/A");

    let since_previous_percent = document
        .select(&since_previous_percent_selector)
        .next()
        .and_then(|el| el.text().next())
        .unwrap_or("N/A");

    // Format the output
    match format.as_str() {
        "price" => println!("Price: {}", price),
        "abs" => println!(
            "Price: {}, Since Open: {} ({}), Since Previous Close: {} ({})",
            price, since_open, since_open_percent, since_previous, since_previous_percent
        ),
        "rel" => println!(
            "Price: {}, Since Open: {}, Since Previous Close: {}",
            price, since_open_percent, since_previous_percent
        ),
        _ => unreachable!("Invalid format"),
    }

    Ok(())
}
