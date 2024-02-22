// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // for i in input.len()
    let mut start_index: usize = 0;
    
    for c in input.chars() {
        if c != ' ' {
            break;
        }
        start_index += 1;
    }
    let length = input.len();
    let mut end_index: usize = length;
    for (_ind, c) in input.char_indices().rev() {
        if c != ' ' {
            break;
        }
        println!("end_index = {end_index}");
        end_index -= 1;
    }
    // println!("start = {}, end = {}", start_index, end_index);
    input[start_index..end_index].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{}{}", input, " world!").to_string()
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let new_input = input.replace("cars", "balloons");
    new_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
