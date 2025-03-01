struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("Name : {} Age : {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("Kenedy Nopriansyah"),
        age: 18,
    };

    println!("{}", person.to_string());
}
