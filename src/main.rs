fn main() {
    let mut y = 10;

    println!("{}", y);

    {
        y = 100;
    }

    println!("{}", y);
}
