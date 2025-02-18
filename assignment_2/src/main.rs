fn is_even(n: i32) -> bool{
    n%2 == 0
}



fn main() {
    let x = vec![1,5,7,9,15,16,18,24,36,79];

    for &num in x.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < x.len() {
        sum += x[i];
        i += 1;
    }
    println!("Sum is equal to: {}", sum);

    let mut max = x[0];
    for num in x {
        if num > max {
            max = num;
        }
    }
    println!("Largest number: {}", max);

}