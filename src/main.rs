use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("create_file.txt").expect("File tidak bisa dibuat");

    file.write_all(b"ini dari youtube channel Kenedy Nopriansyah")
        .expect("Maaf file tidak bisa dibuat");

    let mut f = File::open("create_file.txt").expect("Maaf file tidak ditemukan");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Maaf file tidak bisa dibaca");

    println!("{}", contents);
}
