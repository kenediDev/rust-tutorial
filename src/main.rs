use std::io;

fn main() {
    let mut input = String::new();

    println!("Katakan sesuatu ? ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Dia mengatakan : {}", input);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
