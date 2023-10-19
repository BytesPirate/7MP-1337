struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

fn main() {
    let nums = [2, 7, 11, 15].to_vec();
    let target = 18;

    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}