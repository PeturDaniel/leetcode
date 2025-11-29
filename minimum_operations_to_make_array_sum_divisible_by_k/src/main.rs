struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut total: i32 = nums.iter().sum();
        let mut counter: i32 = 0;
        while total % k != 0 {
            total -= 1;
            counter += 1;
        }
        counter
    }
}

fn main() {
    let tests = vec![(vec![3, 9, 7], 5), (vec![4, 1, 3], 4), (vec![3, 2], 6)];
    for test in tests {
        println!(
            "n -> {:#?} -> {}",
            test.clone(),
            Solution::min_operations(test.0, test.1)
        );
    }
}
