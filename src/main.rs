fn main() {
    let mut x = 10;

    let new_x = &mut x;

    *new_x += 1;

    println!("{}", x);
}
