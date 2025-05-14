use std::time::{Duration, Instant, SystemTime};
use std::thread::sleep;

use reqwest::blocking::Client;
use crate::status::WebsiteStatus;

pub fn check_website(url: &str, timeout_secs: u64, retries: u32) -> WebsiteStatus {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap();

    let mut attempts = 0;
    let start = Instant::now();
    let timestamp = SystemTime::now();

    loop {
        attempts += 1;
        let result = client.get(url).send();
        match result {
            Ok(resp) => {
                let status = resp.status();
                return WebsiteStatus::new(
                    url.to_string(),
                    Ok(status.as_u16()),
                    start.elapsed(),
                    timestamp,
                );
            }
            Err(e) => {
                if attempts > retries {
                    return WebsiteStatus::new(
                        url.to_string(),
                        Err(e.to_string()),
                        start.elapsed(),
                        timestamp,
                    );
                }
                sleep(Duration::from_millis(100));
            }
        }
    }
}
