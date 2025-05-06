struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut rtn_vec: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            rtn_vec.push(nums[nums[i] as usize]);
        }
        rtn_vec
    }
}

fn main() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    println!("{:#?}", Solution::build_array(nums));
    let nums2 = vec![5, 0, 1, 2, 3, 4];
    println!("{:#?}", Solution::build_array(nums2));
}
