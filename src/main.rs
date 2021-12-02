use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() {
    let nums = read().unwrap();
    let ct: i32 = (1..nums.len())
        .map(|i| if &nums[i] > &nums[i - 1] { 1 } else { 0 })
        .sum();
    println!("{}", ct);
}

fn read() -> Result<Vec<i64>, io::Error> {
    let mut path = env::current_dir().expect("failed to get pwd");
    path.push(r"src\test_input.txt");
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line.trim().parse().unwrap();
        v.push(n)
    }
    Ok(v)
}
