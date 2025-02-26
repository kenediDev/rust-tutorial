fn main() {
    let mut tuples = (10, 1.30, "Kenedy Nopriansyah", (90, 9.20, "Kenedy"));

    println!("{:?}", tuples);
    println!("{:?}", (tuples.3).2);

    (tuples.3).2 = "Channel youtube Kenedy Nopriansyah";

    println!("{}", (tuples).2);
    println!("{:?}", tuples)
}
