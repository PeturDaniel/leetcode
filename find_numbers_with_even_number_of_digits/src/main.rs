struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for i in 0..nums.len() {
            if nums[i].to_string().len() % 2 == 0 {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let tests = vec![vec![12, 345, 2, 6, 7896], vec![555, 901, 482, 1771]];
    for n in tests {
        println!("n = {:#?} -> {}", n.clone(), Solution::find_numbers(n));
    }
}
