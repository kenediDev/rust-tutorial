fn main() {
    let mut strings = String::from("Nama saya adalah Kenedy Nopriansyah");

    println!("length {}", strings.len());

    println!("is empty {}", strings.is_empty());

    println!("ini adalah value dari split white space : ");

    for i in strings.split_whitespace() {
        println!("{}", i);
    }

    println!("contains {}", strings.contains("Kenedy"));

    strings.push_str(" dan ini dari youtube Kenedy Nopriansyah");

    println!("{}", strings);
}
