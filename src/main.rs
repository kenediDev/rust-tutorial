enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction: Direction = Direction::Down;

    match direction {
        Direction::Up => println!("ini dari direction up"),
        Direction::Down => println!("ini dari direction down"),
        Direction::Left => println!("ini dari direction left"),
        Direction::Right => println!("ini dari direction right"),
    }
}
