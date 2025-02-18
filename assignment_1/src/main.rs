const FREEZING: i32 = 32;

fn fahrenheit_to_celsius(x: i32) -> i32 {
    (x - FREEZING) * 5 / 9  
}

fn celsius_to_fahrenheit(x: i32) -> i32 {
    (x * 9 / 5) + FREEZING  
}

fn main() {
    let x = 32;
    let y = fahrenheit_to_celsius(x);
    println!("{}", y); 

    for num in x..x + 5 {
        let y = fahrenheit_to_celsius(num);
        println!("{}", y);
    }
} 