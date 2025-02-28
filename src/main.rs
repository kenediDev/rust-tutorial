struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

fn main() {
    let color = Color {
        red: 50,
        blue: 80,
        green: 1,
    };

    print_colors(&color);
    print_colors(&color);
}

fn print_colors(color: &Color) {
    println!(
        "red {} blue {} green {}",
        color.red, color.blue, color.green
    );
}
