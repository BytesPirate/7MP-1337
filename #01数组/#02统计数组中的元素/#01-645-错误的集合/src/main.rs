struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut ans = vec![0, 2];

        nums.sort();

        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                ans[0] = nums[i];
                sum = sum + nums[i] - nums[i+1];
            }else{
                sum = sum + nums[i];
            }
        }

        ans[1] = nums.len() as i32 * (nums.len() as i32 + 1) / 2 - sum - nums[nums.len() - 1];

        return  ans;
    }
}


fn main() {
    let nums = vec![1, 2, 2, 4];

    assert_eq!(Solution::find_error_nums(nums), vec![2, 3]);
}
