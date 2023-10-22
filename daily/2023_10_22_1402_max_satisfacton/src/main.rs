use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort_unstable_by_key(|&x| Reverse(x));
        satisfaction
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                (*state >= 0).then(|| *state)
            })
            .sum()
    }
}

fn main() {
    let satisfaction = vec![-1, -8, 0, 5, -9];

    let ans = Solution::max_satisfaction(satisfaction);
    println!("ans is {:?}", ans);
}
