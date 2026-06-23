struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut curr_alt = 0;
        for num in gain {
            curr_alt += num;
            if curr_alt > ans {
                ans = curr_alt;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![vec![-5, 1, 5, 0, -7], vec![-4, -3, -2, -1, 4, 3, 2]];

    for test in tests {
        println!(
            "{:#?} -> {:#?}",
            test.clone(),
            Solution::largest_altitude(test)
        );
    }
}
