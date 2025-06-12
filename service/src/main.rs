fn main() {
    let mut max_diff = 0;

    let nums: Vec<i32> = vec![1, 2, 9, 4, 5, 6, 7, 8, 9, 10];
    nums.windows(2)
        .for_each(|w| max_diff = max_diff.max(w[0].abs_diff(w[1])));
    println!("max_diff: {max_diff}");
}
