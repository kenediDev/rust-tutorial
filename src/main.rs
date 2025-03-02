fn main() {
    println!(
        "Value {}",
        match option("empty") {
            Some(c) => c,
            None => "Not have value",
        }
    )
}

fn option(value: &str) -> Option<&str> {
    match value {
        "Munich" => Some("This is munich"),
        "spain" => Some("This is spain"),
        _ => None,
    }
}
