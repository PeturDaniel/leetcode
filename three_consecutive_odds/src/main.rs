struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        for i in 0..arr.len() - 2 {
            if arr[i] % 2 != 0 && arr[i + 1] % 2 != 0 && arr[i + 2] % 2 != 0 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let vec1 = vec![2, 6, 4, 1];
    println!("{}", Solution::three_consecutive_odds(vec1));
    let vec2 = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
    println!("{}", Solution::three_consecutive_odds(vec2));
    let vec3 = vec![1];
    println!("{}", Solution::three_consecutive_odds(vec3));
}
