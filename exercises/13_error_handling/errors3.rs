// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8a";

    let cost = total_cost(pretend_user_input);
    match cost {
        Ok(c) => {
            if c > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= c;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(error) => {
            println!("give me number in string instead of number with symbols!");
        }
    }

    Ok(())
}

enum MyOwnResult<T, E> {
    Ok(T),
    Err(E)
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
