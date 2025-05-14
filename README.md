# Spring2025Systems
Assignments for class csci 3334
git add .
git commit-m"completed assignments"
git push origin main



# Website_Status_Checker
Purpose:

This tool was developed as a final project assignment to reinforce understanding of:
- Thread creation & coordination (custom thread pool)
- Ownership & borrowing across threads (safe shared state)
- Error handling using `Result`
- Measuring latency using `std::time::Instant`
- Manual JSON generation
- Command-line parsing
- Basic file and network I/O

## Project Structure: 
src/main.rs: Entry point and orchestration logic

src/args.rs: Argument parsing and config struct

src/checker.rs: Performs HTTP checks

src/status.rs: Status data struct, formatting, and JSON writing

src/threadpool.rs: Fixed-size thread pool implementation


## Build Instructions:

To compile the project in release mode:

```bash
cargo build --release 

## To run the program and examples
cargo run -- [OPTIONS] [URL ...]
cargo run -- https://www.google.com https://www.notreal.com 
cargo run -- --file urls.txt
cargo run -- --file urls.txt https://www.shein.com

## Live Output
[ERROR] https://www.notreal.com: error sending request for url (https://www.notreal.com/): error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1913: (Hostname mismatch) after 77.961ms
[200] https://www.google.com in 95.088844ms

## JSON Output
[
  { "url": "https://www.notreal.com", "status": "error sending request for url (https://www.notreal.com/): error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1913: (Hostname mismatch)", "response_time_ms": 77, "timestamp": 1747239420 },
  { "url": "https://www.google.com", "status": 200, "response_time_ms": 95, "timestamp": 1747239420 }
]

