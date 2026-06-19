struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x == 1 {
            return 1;
        }

        let mut ans = 0;
        let mut low = 1;
        let mut high = x / 2;

        while low <= high {
            let mid = low + (high - low) / 2;
            let square = mid as i64 * mid as i64;
            if square == x as i64 {
                return mid;
            }
            if square < x as i64 {
                ans = mid;
                low = mid + 1;

                continue;
            }

            if square > x as i64 {
                high = mid - 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![4, 8, 1, 2, 3, 2147395599];
    for n in tests {
        println!("n -> {:#?} -> {}", n.clone(), Solution::my_sqrt(n));
    }
}
