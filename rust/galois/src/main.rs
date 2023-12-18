fn main() {
    let a: u8 = 5;
    let b: u8 = 20;
    let p: u8 = 23;
    let x = (a + b) % p;
    let y = (a * b) % p;
    let z = -20 % p;
    println!("a + b mod p = {x}");
    println!("a * b mod p = {y}");
    println!("a - b mod p = {z}");
}

fn additive_inverse(num: i32, p: i32) -> i32 {
}
