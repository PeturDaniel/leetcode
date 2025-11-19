struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let nums_set = nums.iter().collect::<HashSet<_>>();
        let mut rtn_number = original;
        while nums_set.contains(&rtn_number) {
            rtn_number = 2 * rtn_number;
        }
        rtn_number
    }
}

fn main() {
    let tests = vec![(vec![5, 3, 6, 1, 12], 3), (vec![2, 7, 9], 4)];
    for n in tests {
        println!(
            "n -> {:#?} -> {}",
            n.clone(),
            Solution::find_final_value(n.0, n.1)
        );
    }
}
