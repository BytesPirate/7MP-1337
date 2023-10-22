struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans :i32 = 0;

        for i in 0..n{
            ans ^= nums[i];
        }

        let low_bit :i32 = ans & (-ans);

        let mut ans_1 :i32 = 0;
        let mut ans_2 :i32 = 0;

        for i in 0..n {
            if low_bit & nums[i] == 0 {
                ans_1 ^= nums[i];
            }else{
                ans_2 ^= nums[i];
            }
        }

        let mut ans_vec = vec![0;2];
        ans_vec[0] = ans_1;
        ans_vec[1] = ans_2;
        ans_vec
    }
}

fn main() {
    let nums = [1, 2, 1, 3, 2, 5].to_vec();

    let result = Solution::single_number(nums);
    println!("{:?}", result);
}