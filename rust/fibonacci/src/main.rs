fn main() {
    let n = 10;
    println!("fibonacci of {} is: {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    match n {
        0 => panic!("zero is not right argument for fibonacci"),
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
    /*
    // using non-idiomatic rust
    if n == 1 || n == 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
    */
}
