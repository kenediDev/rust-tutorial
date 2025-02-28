struct Color(i32, i32, i32);

fn main() {
    let mut green = Color(255, 0, 0);

    green.2 = 255;

    println!("R {} G {} B {}", green.0, green.1, green.2);
}
