fn main() {
    let animals = ["Cat", "Bird", "Dog"];

    for (index, value) in animals.iter().enumerate() {
        println!("Index dan Nama dari hewan {} : {}", index, value);
    }
}
