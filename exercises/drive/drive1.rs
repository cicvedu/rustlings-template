// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn d1() -> String {
    // to return a string "Hello" 
    // String::from("Hello")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(d1(), "Hello".to_owned());
    }
}
