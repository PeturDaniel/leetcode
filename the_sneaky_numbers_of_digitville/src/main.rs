struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_map = HashSet::new();
        let mut rtn_vec = vec![];
        for num in nums {
            if !nums_map.contains(&num) {
                nums_map.insert(num);
            } else {
                rtn_vec.push(num);
            }
        }

        rtn_vec
    }
}

fn main() {
    let nums1 = vec![0, 1, 1, 0];
    println!("{:#?}", Solution::get_sneaky_numbers(nums1));
    let nums2 = vec![0, 3, 2, 1, 3, 2];
    println!("{:#?}", Solution::get_sneaky_numbers(nums2));
    let nums3 = vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
    println!("{:#?}", Solution::get_sneaky_numbers(nums3));
}
