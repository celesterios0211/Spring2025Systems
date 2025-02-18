
fn sum(total: &mut i32, low: i32, high: i32) {
    let mut i = low; 
    while i <= high {
        *total += i;
        i += 1;
    }
}

fn main() {
    let mut total = 0; 
    let low = 0;
    let high = 100;

    sum(&mut total, low, high); 

    println!("Total sum from {} to {}: {}", low, high, total); 
}
