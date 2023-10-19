struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {

    }
}


fn main() {
    let nums: Vec<i32> = Vec![10, 10, 10, 10, 10];
    let k: i32 = 5;

    let result = Solution::max_kelements(nums, k);

    println!("{}", result);
}