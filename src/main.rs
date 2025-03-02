use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    fullName: String,
    age: u8,
    is_male: bool,
}

fn main() {
    let data = r#"
    {
        "fullName": "Kenedy Nopriansyah",
        "age": 18,
        "is_male": true
    }
        "#;

    let res = serde_json::from_str(data);
    if res.is_ok() {
        let result: Person = res.unwrap();
        println!(
            "Name : {}, Age : {}, Is Male : {}",
            result.fullName, result.age, result.is_male
        );
    } else {
        println!("Data tidak dapat diparse");
    }
}
