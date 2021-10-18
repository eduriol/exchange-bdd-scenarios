pub fn default_string() -> String {
    "something".to_string()
}

pub fn particular_value() -> String {
    "interesting".to_string()
}

pub fn interesting_appendage(input: &str) -> String {
    format!("{} {}", input, particular_value())
}
