fn main() {
    println!("{}", print_numbers_to(32));

    arr_func(10);
}

fn print_numbers_to(value: i32) -> i32 {
    return value;
}

fn arr_func(num: i32) {
    println!("ini value dari arr func");
    for i in 0..num {
        println!("{}", i + 1);
    }
}
