use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the ISIN from the user
    println!("Enter the ISIN:");
    let mut isin = String::new();
    std::io::stdin().read_line(&mut isin)?;
    let isin = isin.trim();

    // Construct the URL
    let url = format!("https://live.euronext.com/en/ajax/getDetailedQuote/{}-XAMS", isin);

    // Fetch the HTML from the URL
    let client = Client::new();
    let response = client.get(&url).send()?.text()?;
    
    // Parse the HTML
    let document = Html::parse_document(&response);

    // Extract the required data
    let price_selector = Selector::parse("#header-instrument-price").unwrap();
    let since_open_selector = Selector::parse("div:contains('Since Open') + span").unwrap();
    let since_open_percent_selector = Selector::parse("div:contains('Since Open') + span + span").unwrap();
    let since_previous_selector = Selector::parse("div:contains('Since Previous Close') + span").unwrap();
    let since_previous_percent_selector = Selector::parse("div:contains('Since Previous Close') + span + span").unwrap();

    // Extract the data
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

    // Print the results
    println!("Price: {}", price);
    println!("Since Open: {} ({})", since_open, since_open_percent);
    println!("Since Previous Close: {} ({})", since_previous, since_previous_percent);

    Ok(())
}
