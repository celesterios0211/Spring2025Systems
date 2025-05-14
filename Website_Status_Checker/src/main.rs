mod args;
mod checker;
mod status;
mod threadpool;

use crate::args::parse_args;
use crate::checker::check_website;
use crate::status::{write_json};
use crate::threadpool::ThreadPool;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self};
use std::sync::{Arc, Mutex};

fn read_urls_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .collect()
}

fn main() {
    let config = parse_args();

    let mut urls = config.urls.clone();
    if let Some(ref file_path) = config.file {
        let mut file_urls = read_urls_from_file(&file_path);
        urls.append(&mut file_urls);
    }

    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file <file>] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    let (tx, rx) = mpsc::channel();
    let results = Arc::new(Mutex::new(Vec::new()));
    let pool = ThreadPool::new(config.workers);

    for url in urls {
        let tx = tx.clone();
        let url = url.clone();
        let results = Arc::clone(&results);
        let config = config.clone();
        pool.execute(move || {
            let result = check_website(&url, config.timeout_secs, config.retries);
            println!("{}", result.human_readable());
            results.lock().unwrap().push(result.clone());
            tx.send(()).unwrap();
        });
    }

    // Wait for all tasks
    for _ in 0..config.urls.len() {
        rx.recv().unwrap();
    }

    // Save results to JSON
    let final_results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    write_json(&final_results, "status.json").expect("Failed to write status.json");
}
