use std::collections::HashMap;

fn digit_sum(mut x: i32) -> i32 {
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

struct Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut counts = HashMap::new();
        for i in 1..=n {
            let s = digit_sum(i);
            *counts.entry(s).or_insert(0) += 1;
        }
        let max_size = counts.values().copied().max().unwrap_or(0);
        return counts
            .values()
            .filter(|&&c| c == max_size)
            .count()
            .try_into()
            .unwrap();
    }
}

fn main() {
    let tests = [1, 2, 13, 24, 1000];
    for &n in &tests {
        println!("n = {:>3} -> {}", n, Solution::count_largest_group(n));
    }
}
