fn main() {
    // cesar cipher decoder
    let s: String = "uhtlishjrvbadvyyplkaohavbyjp".to_string();
    let enc = encrypt(&s, 7);
    let dec = decrypt(&s, 19);
    generate_all_plain_text(&s);
    println!("message: {}\nencrypted message: {:?}\ndecrypted message: {:?}", s, enc, dec);
}

// TODO: send search to a dictionary for match of generated text
// TODO: use frequency method to find the key

fn generate_all_plain_text(_s: &str) -> Vec<String> {
    let chars: Vec<char> = _s.to_lowercase().chars().collect();
    let mut _r: Vec<String> = vec![];
    for i in 0..=26 {
        let mut tmp = String::new();
        for c in chars.iter() {
            tmp.push((((*c as u8 - 97 + i) % 26) + 97) as char);
        }
        _r.push(tmp.clone());
    }
    println!("generated texts: {:?}", _r);
    _r
}

fn encrypt(_s: &String, _shift: u8) -> String {
    let chars: Vec<char> = _s.to_lowercase().chars().collect();
    let mut cipher: String = String::new();
    for c in chars.iter() {
        cipher.push((((*c as u8 - 97 + _shift) % 26) + 97) as char);
    }
    cipher
}

fn decrypt(_s: &String, _shift: u8) -> String {
    // TODO: additive inverse for shift key
    let chars: Vec<char> = _s.to_lowercase().chars().collect();
    let mut cipher: String = String::new();
    for c in chars.iter() {
        cipher.push((((*c as u8 - 97 + _shift) % 26) + 97) as char);
    }
    cipher
}
