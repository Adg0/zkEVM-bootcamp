fn main() {
    // Fahrenheit to celsius
    let tempreature: f32 = 69.8;
    println!("{:.1} fahrenheit tempreature in celsius is: {:.1}", tempreature, fahrenheit_celsius(tempreature, 'c'));
}

fn fahrenheit_celsius(temp: f32, to: char) -> f32 {
    match to {
        'c' => (temp - 32.0) * 5.0/9.0,
        'f' => (temp * 9.0/5.0) + 32.0,
        _ => 0.0,
    }
}
