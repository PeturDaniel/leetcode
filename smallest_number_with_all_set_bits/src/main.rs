struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut rtn_num: i32 = 1;
        while rtn_num <= n {
            rtn_num = rtn_num * 2;
        }
        rtn_num - 1
    }
}

fn main() {
    let nums: i32 = 5;
    println!("{:#?}", Solution::smallest_number(nums));
    let nums2: i32 = 10;
    println!("{:#?}", Solution::smallest_number(nums2));
    let nums3: i32 = 3;
    println!("{:#?}", Solution::smallest_number(nums3));
    let nums4: i32 = 0;
    println!("{:#?}", Solution::smallest_number(nums4));
}
