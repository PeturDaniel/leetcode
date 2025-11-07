struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |w| w.len() as i32)
    }
}

fn main() {
    let tests = [
        "Hello World",
        "   fly me   to   the moon  ",
        "luffy is still joyboy",
    ];
    for &n in &tests {
        println!(
            "n = {:>3} -> {}",
            n,
            Solution::length_of_last_word(n.to_string())
        );
    }
}
