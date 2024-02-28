// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

enum MuOwnOption<T> {
    None,
    Some(T),
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if a == 0.0 {
        None
    }
    else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_code() {
        assert_eq!(0, 0);
        let result = divide(2.0, 0.0);

        match result {
            Some(x) => println!("result: {x}"),
            None => println!("Cannot divide by 0"),
        }

        let s = String::from("123");
        println!("============{}=================", s.is_empty());

        let s = "1234".into();
        let res = generate_nametag_text(s);
        println!("{:?}", res);
        // assert_eq!(s, Some(String::from("1234")));
    }

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
