fn main() {
    /* Replace */
    {
        let strings = String::from("Rust is fantastic!");
        println!("{}", strings.replace("fantastic", "great"));
    }
    /* Lines */
    {
        let strings = String::from(
            "Kenedy Nopriansyah on Youtube \n Dont Forgot Subscribe \n Likes \n Notification",
        );
        for i in strings.lines() {
            println!("[ {} ]", i);
        }
    }
    /* Trim */
    {
        let strings = String::from("   Kenedy Nopriansyah on Youtube");

        println!("before trim {}", strings);
        println!("after trim {}", strings.trim());
    }
    /* Chars */
    {
        let strings = String::from("Kenedy Nopriansyah on Youtube");
        match strings.chars().nth(4) {
            Some(c) => println!("Ya disana ada value dan tidak empty {}", c),
            None => println!("Value is empty"),
        }
    }
    {
        let strings = String::from("Kenedy+Nopriansyah+on+youtube");
        let split: Vec<&str> = strings.split("+").collect();
        for i in split.iter() {
            println!("{}", i);
        }
    }
}
