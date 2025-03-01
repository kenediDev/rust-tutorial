fn main() {
    let mut vector = vec![100, 23, 55, 92, 52];

    vector.remove(1);
    vector.push(999);

    for i in 0..vector.len() {
        println!("index {}, value {}", i, vector[i]);
    }
}
