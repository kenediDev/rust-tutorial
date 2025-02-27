fn main() {
    let mut n = 10;

    {
        let n = 12;

        println!("{}", n);
    }

    n = 100;

    println!("{}", n);
}
