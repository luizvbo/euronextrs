use clap::{Arg, Command};
use csv::Writer;
use serde_json::Value;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("euronextrs")
        .arg(Arg::new("OUTPUT_PATH")
            .help("Sets the output path for the CSV file")
            .required(true)
            .index(1))
        .arg(Arg::new("ISIN")
            .help("ISIN with stock exchange code (as given in the Euronext URL)")
            .default_value("IE00B4L5Y983-XAMS")
            .index(2))
        .get_matches();

    let url = format!("https://live.euronext.com/intraday_chart/getChartData/{}/intraday", matches.get_one::<String>("ISIN").unwrap());
    let output_path = PathBuf::from(matches.get_one::<String>("OUTPUT_PATH").unwrap());

    fetch_and_print(&url, output_path).await
}

async fn fetch_and_print(url: &str, output_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let json: Vec<Value> = response.json().await?;

    let mut writer = Writer::from_path(output_path)?;

    writer.write_record(&["time", "price", "volume"])?;
    for item in &json {
        if let Value::Object(map) = item {
            writer.write_record(&[
                map.get("time").and_then(Value::as_str).unwrap_or_default().to_string(),
                map.get("price").and_then(Value::as_f64).unwrap_or_default().to_string(),
                map.get("volume").and_then(Value::as_u64).unwrap_or_default().to_string(),
            ])?;
        }
    }

    writer.flush()?;

    Ok(())
}
