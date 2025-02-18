fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}


fn main() {
    let secret = 28; 
    let mut attempts = 0;

    loop {
        attempts += 1;
        let guess = 15 + attempts; 

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("The number {} is correct!", secret);
            break;
        } else if result == 1 {
            println!("The number {} is too high!", guess);
        } else {
            println!("The number {} is too low!", guess);
        }
    }

    println!("It took {} attempts to guess the number.", attempts);
}
