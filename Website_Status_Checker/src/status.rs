use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

impl WebsiteStatus {
    pub fn new(
        url: String,
        status: Result<u16, String>,
        response_time: Duration,
        timestamp: SystemTime,
    ) -> Self {
        WebsiteStatus {
            url,
            status,
            response_time,
            timestamp,
        }
    }

    pub fn human_readable(&self) -> String {
        match &self.status {
            Ok(code) => format!(
                "[{}] {} in {:?}",
                code, self.url, self.response_time
            ),
            Err(err) => format!(
                "[ERROR] {}: {} after {:?}",
                self.url, err, self.response_time
            ),
        }
    }
}

pub fn write_json(statuses: &[WebsiteStatus], path: &str) -> IoResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "[")?;
    for (i, s) in statuses.iter().enumerate() {
        let json = format!(
            "  {{ \"url\": \"{}\", \"status\": {}, \"response_time_ms\": {}, \"timestamp\": {:?} }}{}",
            s.url,
            match &s.status {
                Ok(code) => code.to_string(),
                Err(err) => format!("\"{}\"", err),
            },
            s.response_time.as_millis(),
            s.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            if i + 1 != statuses.len() { "," } else { "" }
        );
        writeln!(file, "{}", json)?;
    }
    writeln!(file, "]")?;
    Ok(())
}
