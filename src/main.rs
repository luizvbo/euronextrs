use clap::Parser;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

/// Extracts financial data from the Euronext website
#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    /// The ISIN of the instrument to fetch data for
    #[arg(value_name = "ISIN")]
    isin: String,

    /// Output format string
    /// Use %p for price, %o for since open, %O for since open percentage,
    /// %c for since close, %C for since close percentage
    #[arg(short, long, value_name = "FORMAT", default_value = "%p | Open: %o (%O) | Close: %c (%C)")]
    format: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line arguments using the derive API
    let args = Args::parse();

    // Get the ISIN and format from arguments
    let isin = args.isin;
    let format = args.format;

    // Construct the URL
    let url = format!(
        "https://live.euronext.com/en/ajax/getDetailedQuote/{}",
        isin.to_uppercase()
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
    let since_close_percent = since_close / price;

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

#[cfg(test)]
mod tests {
    use super::*;

    // Test the formatting logic
    #[test]
    fn test_formatting() {
        let price = 100.0;
        let since_open = 98.0;
        let since_open_percent = 2.0; // 2%
        let since_close = 99.0;
        let since_close_percent = 1.0; // 1%

        let format_string = "%p - Open: %o (%O) - Close: %c (%C)";
        let formatted_output = format_output(
            format_string,
            price,
            since_open,
            since_open_percent,
            since_close,
            since_close_percent,
        );

        let expected_output = "100 - Open: 98 (2.00%) - Close: 99 (1.00%)";
        assert_eq!(formatted_output, expected_output);
    }

    // Test the output with various format strings
    #[test]
    fn test_format_with_custom_format() {
        let price = 100.0;
        let since_open = 98.0;
        let since_open_percent = 2.0; // 2%
        let since_close = 99.0;
        let since_close_percent = 1.0; // 1%

        let format_string = "Price: %p | Open: %o | Open %: %O";
        let formatted_output = format_output(
            format_string,
            price,
            since_open,
            since_open_percent,
            since_close,
            since_close_percent,
        );

        let expected_output = "Price: 100 | Open: 98 | Open %: 2.00%";
        assert_eq!(formatted_output, expected_output);
    }

    // Mock test for the extract data function
    #[test]
    fn test_extract_data_from_html() {
        // This test simulates the extraction of price data from HTML content.
        let html_content = r#"
            <html>
                <div id="header-instrument-price">100.5</div>
                <div class="data-header__col-right">
                    <span class="data-24">98.3</span>
                    <span class="text-ui-grey-1">-2.0%</span>
                </div>
            </html>
        "#;

        let document = Html::parse_document(html_content);
        let price_selector = Selector::parse("#header-instrument-price").unwrap();
        let price = extract_price(&document, &price_selector);

        assert_eq!(price, 100.5);
    }

    // Utility function for formatting the output
    fn format_output(
        format: &str,
        price: f64,
        since_open: f64,
        since_open_percent: f64,
        since_close: f64,
        since_close_percent: f64,
    ) -> String {
        format
            .replace("%p", &price.to_string())
            .replace("%o", &since_open.to_string())
            .replace("%O", &format!("{:.2}%", since_open_percent))
            .replace("%c", &since_close.to_string())
            .replace("%C", &format!("{:.2}%", since_close_percent))
    }

    // Helper function for extracting price (mocking a real extraction)
    fn extract_price(document: &scraper::Html, selector: &scraper::Selector) -> f64 {
        document
            .select(selector)
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("0")
            .replace(",", "")
            .parse::<f64>()
            .unwrap_or(0.0)
    }
}
