struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut rtn_num: i32 = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                let (left, right) = nums.split_at(i);
                let left_sum: i32 = left.iter().sum();
                let right_sum: i32 = right.iter().sum();
                if right_sum - left_sum == 0 {
                    rtn_num += 2;
                } else if (right_sum - left_sum).abs() == 1 {
                    rtn_num += 1;
                }
            }
        }

        rtn_num
    }
}

fn main() {
    let nums = vec![1, 0, 2, 0, 3];
    println!("{:#?}", Solution::count_valid_selections(nums));
    let nums2 = vec![2, 3, 4, 0, 4, 1, 0];
    println!("{:#?}", Solution::count_valid_selections(nums2));
}
