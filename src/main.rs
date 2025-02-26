fn main() {
    let t: (i32, f32, String, bool) = (32, 2.32, "Hahaha".to_string(), false);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3];

    println!("{:?}\n{:?}\n{:?}", t, arr, slice);
}
