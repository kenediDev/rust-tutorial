enum Day {
    Senin,
    Selesa,
    Rabu,
    Kami,
    Jumat,
    Sabtu,
    Minggu,
}

impl Day {
    fn is_weekday(&self) {
        match self {
            &Day::Senin | &Day::Rabu => println!("Ya ini adalah hari senin atau rabu"),
            _ => println!("ini adalah hari weekend"),
        }
    }
}

fn main() {
    let day = Day::Minggu;

    day.is_weekday();
}
