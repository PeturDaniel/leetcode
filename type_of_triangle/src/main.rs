struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] <= nums[2]
            || nums[0] + nums[2] <= nums[1]
            || nums[1] + nums[2] <= nums[0]
        {
            return "none".try_into().unwrap();
        }
        if nums[0] == nums[1] && nums[0] == nums[2] {
            return "equilateral".try_into().unwrap();
        }
        if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] {
            return "isosceles".try_into().unwrap();
        }
        return "scalene".try_into().unwrap();
    }
}

fn main() {
    let vec1 = vec![3, 3, 3];
    println!("{}", Solution::triangle_type(vec1));
    let vec2 = vec![3, 4, 3];
    println!("{}", Solution::triangle_type(vec2));
    let vec3 = vec![3, 4, 5];
    println!("{}", Solution::triangle_type(vec3));
    let vec4 = vec![8, 4, 2];
    println!("{}", Solution::triangle_type(vec4));
}
