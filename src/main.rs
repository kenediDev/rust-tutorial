struct Person {
    name: String,
    age: u8,
}

trait TraitPerson {
    fn print_to(&self);
    fn is_square(&self) -> bool;
}

impl TraitPerson for Person {
    fn print_to(&self) {
        println!("Name {} Age {}", self.name, self.age);
    }

    fn is_square(&self) -> bool {
        return self.age > 19;
    }
}

fn main() {
    let person = Person {
        name: String::from("Kenedy Nopriansyah"),
        age: 18,
    };

    person.print_to();

    println!("{}", person.is_square());
}
