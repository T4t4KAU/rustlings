#![allow(clippy::ptr_arg)]

// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&mut data);
}