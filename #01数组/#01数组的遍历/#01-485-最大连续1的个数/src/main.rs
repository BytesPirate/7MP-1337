struct  Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                count = count + 1;
                if ans > count {
                    ans = ans;
                }else{
                    ans = count;
                }
            }else{
                count = 0
            }
        }

        return ans;
    }
}
fn main() {
    let nums = vec![1,1,0,1,1,1];

    println!("{}", Solution::find_max_consecutive_ones(nums));
}
