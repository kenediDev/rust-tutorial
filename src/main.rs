mod kenednopriansyahOnYoutube {
    fn another_function() {
        println!("This is another func");
    }

    pub fn to_string() {
        another_function();
        println!("Hai ini dari modules");
    }

    pub mod second {
        pub fn print_message() {
            println!("Hello All");
        }
    }
}

fn main() {
    kenednopriansyahOnYoutube::second::print_message();
}
