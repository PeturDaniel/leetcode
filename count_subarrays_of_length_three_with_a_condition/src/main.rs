struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for i in 0..nums.len() - 2 {
            let sum: i32 = nums[i] + nums[i + 2];
            if sum * 2 == nums[i + 1] {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let tests = vec![vec![1, 2, 1, 4, 1], vec![1, 1, 1], vec![-1, -4, -1, 4]];
    for n in tests {
        println!("n = {:#?} -> {}", n.clone(), Solution::count_subarrays(n));
    }
}
