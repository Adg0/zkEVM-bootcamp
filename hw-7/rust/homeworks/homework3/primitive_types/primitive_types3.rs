// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `zustlings hint primitive_types3` for hints!

// I AM DONE

fn main() {
    let a:[u8; 100] = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
