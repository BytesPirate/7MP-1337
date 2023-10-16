struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    }
}

fn main() {
    let nums = [2, 7, 11, 15].to_vec();
    let target = 18;

    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}