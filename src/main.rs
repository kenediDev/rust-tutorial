fn main() {
    let s = "Kenedy Nopriansyah";

    match s {
        "Kenedy" | "Kenedy Nopriansyah" => println!("Sama sedang Kenedy atau Kenedy Nopriansyah"),
        "Nopriansyah" => println!("Sama dengan Nopriansyah"),
        _ => println!("Tidak ada yang cocok"),
    }
}
