use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::thread;
use ureq:: serde_json;

// Trait remains the same
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self) -> Result<(), String>;
}

// Struct for Bitcoin
#[derive(Debug, Deserialize)]
struct Bitcoin {
    price: f64,
}

// Struct for Ethereum
#[derive(Debug, Deserialize)]
struct Ethereum {
    price: f64,
}

// Struct for SP500
#[derive(Debug, Deserialize)]
struct SP500 {
    price: f64,
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

        let response = ureq::get(url).call().map_err(|e| format!("Network error: {}", e))?;

        if response.status() == 200 {
            // Directly parse the JSON into the Bitcoin struct
            let parsed: serde_json::Value = response
                .into_json()
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            // Extract price directly from the parsed JSON
            self.price = parsed["bitcoin"]["usd"]
                .as_f64()
                .ok_or("Failed to extract price")?;
            
            Ok(())
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = File::create("bitcoin.txt").map_err(|e| e.to_string())?;
        write!(file, "Bitcoin Price: ${}", self.price).map_err(|e| e.to_string())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";

        let response = ureq::get(url).call().map_err(|e| format!("Network error: {}", e))?;

        if response.status() == 200 {
            // Directly parse the JSON into the Bitcoin struct
            let parsed: serde_json::Value = response
                .into_json()
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            // Extract price directly from the parsed JSON
            self.price = parsed["ethereum"]["usd"]
                .as_f64()
                .ok_or("Failed to extract price")?;
            
            Ok(())
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = File::create("ethereum.txt").map_err(|e| e.to_string())?;
        write!(file, "Ethereum Price: ${}", self.price).map_err(|e| e.to_string())
    }
}

impl Pricing for SP500{
    fn fetch_price(&mut self) -> Result<(), String> {
        let api_key = "zj1E0ai9KtIF0Ec6DM64X1L0yNFOZzOo"; // Replace with your actual API key
        let url = format!(
            "https://financialmodelingprep.com/api/v3/quote/SPY?apikey={}",
            api_key
        );

        let response = ureq::get(&url).call().map_err(|e| format!("Network error: {}", e))?;

        if response.status() == 200 {
            let parsed: serde_json::Value = response
                .into_json()
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            self.price = parsed[0]["price"]
                .as_f64()
                .ok_or("Failed to extract SP500 price")?;

            Ok(())
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = File::create("sp500.txt").map_err(|e| e.to_string())?;
        write!(file, "S&P 500 Price: ${}", self.price).map_err(|e| e.to_string())
    }
}

fn main() {
    let mut bitcoin = Bitcoin { price: 0.0 };
    let mut ethereum = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        println!("📊 Fetching prices...\n");

        let assets: &mut [(&str, &mut dyn Pricing)] = &mut [
            ("Bitcoin", &mut bitcoin),
            ("Ethereum", &mut ethereum),
            ("S&P 500", &mut sp500),
        ];

        for (name, asset) in assets {
            println!("Fetching {name} price...");

            match asset.fetch_price() {
                Ok(_) => {
                    println!("{name} fetched successfully.");
                    if let Err(e) = asset.save_to_file() {
                        println!("Failed to save {name}: {e}");
                    }
                }
                Err(e) => {
                    println!("Failed to fetch {name}: {e}");
                }
            }

            println!();
        }

        println!("Waiting 10 seconds...\n");
        thread::sleep(Duration::from_secs(10));
    }
}