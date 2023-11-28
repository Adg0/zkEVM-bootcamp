fn main() {
    // Welcome message
    println!("Welcome to Fizz Buzz zkEVM bootcamp!");

    // Calling fizz_buzz function
    fizz_buzz();
}

fn fizz_buzz() {
    // mutable variable to keep track of `fizz buzz` count
    let mut count: u8 = 0;

    // loop upto 301
    for i in 0..=301 {

        // match based on divisible by 3 and 5
        match (i % 3, i % 5) {
            (0, 0) => {
                count += 1; // increment count when `fizz buzz`
                println!("fizz buzz");
            },
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", i),
        }
    }
    println!("Count of `fizz buzz` is: {}", count);
}

// Unoptimzed/non-Idiomatic fizz_buzz using if and else if
/*
fn fizz_buzz2() {
    // loop counting upto 301: 0..=301
    // if divisible by 3 print "fizz"
    // if divisible by 5 print "buzz"
    // if divisible by both 3 and 5 print "fizz buzz"
    // print number of times "fizz buzz" occurred
    let mut count: u8 = 0;
    for i in 0..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            count += 1;
            println!("fizz buzz"); 
        } else if i % 3 == 0 {
           println!("fizz"); 
        } else if i % 5 == 0 {
           println!("buzz"); 
        } else {
            println!("{}", i);
        }
    }
    println!("fizz buzz count: {}", count);
}
*/

