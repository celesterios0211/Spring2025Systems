use std::env;

#[derive(Clone)]
pub struct Config {
    pub file: Option<String>,
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout_secs: u64,
    pub retries: u32,
}

pub fn parse_args() -> Config {
    let mut args = env::args().skip(1);
    let mut file = None;
    let mut urls = Vec::new();
    let mut workers = num_cpus::get(); // defaults to logical cores
    let mut timeout_secs = 5;
    let mut retries = 0;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" => file = args.next(),
            "--workers" => workers = args.next().unwrap().parse().unwrap_or(workers),
            "--timeout" => timeout_secs = args.next().unwrap().parse().unwrap_or(5),
            "--retries" => retries = args.next().unwrap().parse().unwrap_or(0),
            _ => urls.push(arg),
        }
    }

    Config {
        file,
        urls,
        workers,
        timeout_secs,
        retries,
    }
}
