use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let rtn_vec = nums
            .windows(k as usize)
            .map(|w| Self::calculate_x_sum(w, x))
            .collect();

        rtn_vec
    }

    pub fn calculate_x_sum(nums: &[i32], x: i32) -> i32 {
        let mut nums_count: HashMap<i32, i32> = HashMap::new();
        for &num in nums {
            *nums_count.entry(num).or_insert(0) += 1;
        }

        let mut count_vec: Vec<(i32, i32)> = nums_count.into_iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        let max_n = x.min(count_vec.len() as i32);

        let mut rtn_num = 0;
        for i in 0..max_n as usize {
            rtn_num += count_vec[i].0 * count_vec[i].1;
        }

        rtn_num
    }
}

fn main() {
    let test1 = vec![1, 1, 2, 2, 3, 4, 2, 3];
    let k1 = 6;
    let x1 = 2;
    println!("{:#?}", Solution::find_x_sum(test1, k1, x1));
    let test2 = vec![3, 8, 7, 8, 7, 5];
    let k2 = 2;
    let x2 = 2;
    println!("{:#?}", Solution::find_x_sum(test2, k2, x2));
}
