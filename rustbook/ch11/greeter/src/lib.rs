pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // adding a custom message to be printed with the failure message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {result}"
        );
    }
}
