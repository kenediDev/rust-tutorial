struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

fn main() {
    let mut color = Color {
        red: 100,
        blue: 200,
        green: 300,
    };

    color.green = 50;

    println!(
        "red : {} blue : {} green : {}",
        color.red, color.blue, color.green
    );
}
