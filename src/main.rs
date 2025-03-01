struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("width {} height {}", self.width, self.height);
    }
    fn is_square(&self) -> bool {
        self.width != self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 100,
        height: 100,
    };

    rectangle.print_description();
    println!("{}", rectangle.is_square());
}
