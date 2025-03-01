use rand::prelude::*;

fn main() {
    let random = rand::rng().random_range(0..=10);
    println!("{}", random);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    arr.shuffle(&mut rand::rng());

    println!("{:?}", arr);
}
