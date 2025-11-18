struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut length = 0;

        while length < bits.len() {
            if length == bits.len() - 1 && bits[length] == 0 {
                return true;
            }
            if bits[length] == 0 {
                length += 1;
                continue;
            }
            if bits[length] == 1 {
                length += 2;
                continue;
            }
        }

        false
    }
}

fn main() {
    let tests = vec![
        vec![1, 0, 0],
        vec![1, 1, 1, 0],
        vec![1, 1, 1, 0, 0],
        vec![0],
        vec![1],
    ];
    for n in tests {
        println!(
            "n -> {:#?} -> {}",
            n.clone(),
            Solution::is_one_bit_character(n)
        );
    }
}
