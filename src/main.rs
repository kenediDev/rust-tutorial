fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple: (i32, f32, bool, String) = (12, 1.12, false, "Kenedy Nopriansyah".to_string());
    let slice: &[i32] = &arr[0..3];

    println!("{:?}", arr);
    println!("{:?}", tuple);
    println!("{:?}", slice);
}
