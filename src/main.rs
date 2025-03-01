use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 40);
    marks.insert("Web Development", 90);
    marks.insert("UX Design", 95);
    marks.insert("Proffersional Computing Studies", 20);

    println!("Length {}", marks.len());
    println!("{:?}", marks);

    marks.remove("UX Design");

    println!("New HashMap : {:?}", marks);

    marks.insert("New UX Design", 100);

    println!("New HashMap Add UX Design : {:?}", marks);

    for (key, value) in &marks {
        println!("Key : {}, Value {}", key, value);
    }
}
