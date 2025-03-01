use regex::Regex;

fn main() {
    let regex = Regex::new(r"\w{6}").unwrap();

    match regex.captures("Kened") {
        Some(c) => println!("{}", &c[0]),
        None => println!("Regex tidak sesuai"),
    }
}
