struct Solution;

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut new_num1 = num1.clone();
        let mut new_num2 = num2.clone();
        let mut counter: i32 = 0;
        while new_num1 != 0 && new_num2 != 0 {
            if new_num1 >= new_num2 {
                new_num1 -= new_num2;
            } else {
                new_num2 -= new_num1;
            }
            counter += 1;
        }
        counter
    }
}

fn main() {
    let tests = vec![vec![2, 3], vec![10, 10]];
    for n in tests {
        println!(
            "n = {:#?} -> {}",
            n.clone(),
            Solution::count_operations(n[0], n[1])
        );
    }
}
