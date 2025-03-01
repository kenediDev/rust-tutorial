pub fn to_string() {
    let s = String::from("Kenedy Nopriansyah+On+Youtube");
    let strings: Vec<&str> = s.split("+").collect();
    for i in strings.iter() {
        println!("{}", i);
    }
}

fn is_private() {
    println!("This is private function");
}
