struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut available = [0; 10];
        for digit in digits {
            available[digit as usize] += 1;
        }

        let mut result = Vec::new();

        for i in (100..1000).step_by(2) {
            let needed = Self::count_digits(i);

            if needed
                .iter()
                .zip(available.iter())
                .all(|(&need, &have)| need <= have)
            {
                result.push(i);
            }
        }

        result
    }

    fn count_digits(mut num: i32) -> [i32; 10] {
        let mut freq = [0; 10];
        while num > 0 {
            freq[(num % 10) as usize] += 1;
            num /= 10;
        }
        freq
    }
}

fn main() {
    let vec1 = vec![2, 1, 3, 0];
    println!("{:#?}", Solution::find_even_numbers(vec1));
    let vec2 = vec![2, 2, 8, 8, 2];
    println!("{:#?}", Solution::find_even_numbers(vec2));
    let vec3 = vec![3, 7, 5];
    println!("{:#?}", Solution::find_even_numbers(vec3));
}
