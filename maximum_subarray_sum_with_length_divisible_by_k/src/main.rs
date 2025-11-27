struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut prefix: i64 = 0;
        let kk = k as usize;
        let mut min_prefix = vec![i64::MAX; kk];
        min_prefix[0] = 0;
        let mut best: i64 = i64::MIN;
        for i in 0..nums.len() {
            prefix += nums[i] as i64;
            let index = i + 1;
            let r = index % kk;
            if min_prefix[r] != i64::MAX {
                let candidate = prefix - min_prefix[r];
                if candidate > best {
                    best = candidate;
                }
            }
            if prefix < min_prefix[r] {
                min_prefix[r] = prefix;
            }
        }
        best
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2], 1),
        (vec![-1, -2, -3, -4, -5], 4),
        (vec![-5, 1, 2, -3, 4], 2),
        (vec![-10, -1], 1),
    ];
    for n in tests {
        println!(
            "n -> {:#?} -> {}",
            n.clone(),
            Solution::max_subarray_sum(n.0, n.1)
        );
    }
}
