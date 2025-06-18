pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let n = nums.len();
    let res: Vec<Vec<i32>> = nums
        .chunks(3)
        .filter(|chunk| chunk[2] - chunk[0] <= k)
        .map(|chunk| chunk.to_vec())
        .collect();
    if res.len() == n / 3 { res } else { vec![] }
}
fn main() {
    println!("Hello, world!");
}
