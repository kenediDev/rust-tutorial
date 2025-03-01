use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("nama_file.txt").expect("File tersebut tidak ada");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("File tidak ditemukan");

    println!("{}", contents);
}
