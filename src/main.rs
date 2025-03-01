use std::env;

fn main() {
    let argument: Vec<String> = env::args().collect();

    for i in argument.iter() {
        println!("{}", i);
    }
}
