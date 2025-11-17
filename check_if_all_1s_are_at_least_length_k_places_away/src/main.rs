struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut counter = 0;
        let mut seen_1 = false;
        for i in 0..nums.len() {
            if nums[i] == 1 && seen_1 == false {
                seen_1 = true;
            } else if nums[i] == 1 && seen_1 == true {
                if counter < k {
                    return false;
                }
                counter = 0;
            } else if seen_1 {
                counter += 1;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 0, 0, 1, 0, 0, 1], 2),
        (vec![1, 0, 0, 1, 0, 1], 2),
        (vec![0, 1, 0, 0, 1, 0, 0, 1], 2),
        (vec![1, 1, 1, 0], 3),
    ];
    for n in tests {
        println!(
            "n = {:#?} -> {}",
            n.clone(),
            Solution::k_length_apart(n.0, n.1)
        );
    }
}
