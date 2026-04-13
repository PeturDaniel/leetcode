use std::i32::MAX;

struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut answer: i32 = MAX;
        for i in 0..nums.len() {
            if nums[i] == target {
                let new = (i as i32 - start).abs();
                if new < answer {
                    answer = new;
                }
            }
        }

        answer
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], 5, 3),
        (vec![1], 1, 0),
        (vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
    ];
    for n in tests {
        println!(
            "n = {:#?} -> {}",
            n.clone(),
            Solution::get_min_distance(n.0, n.1, n.2)
        );
    }
}
