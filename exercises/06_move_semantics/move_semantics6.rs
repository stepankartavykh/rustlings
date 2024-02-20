// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
// fn print

fn main() {
    println!("=========================");

    let data = "Rust is great!".to_string();
    // data.push_str("qwe");
    let c = get_char(&data);
    println!("{}", c);

    string_uppercase(data);

    println!("=========================");
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
