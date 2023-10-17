/*
    Author: J1n H4ng
    Date: 2023/10/17
    Program: 2652. Sum of Multiples
    Link: https://leetcode.cn/problems/sum-multiples/description/
    Description: 1 到 n 中，3，5，7的整数倍之和
 */
struct Solution;

impl Solution {
    // Solution 1
    // pub fn sum_of_multiples(n: i32) -> i32 {
    //     let n = n;
    //     let mut ans = 0;
    //
    //     for i in 1..(n+1) {
    //         if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
    //             println!("i: {}", i);
    //             ans += i;
    //             println!("ans: {}", ans)
    //         }
    //     }
    //
    //     return ans;
    // }

    // Solution 2
    pub fn sum_of_multiples(n: i32) -> i32 {
        let n = n + 1;
        let sum = Self::sum_multiples(n, 3) + Self::sum_multiples(n, 5) + Self::sum_multiples(n, 7) - Self::sum_multiples(n, 15) - Self::sum_multiples(n, 21) - Self::sum_multiples(n, 35) + Self::sum_multiples(n, 105);
        return sum;
    }

    pub fn sum_multiples(n: i32, m: i32) -> i32 {
        let p = (n -1) / m;
        return m * (p * (p + 1)) / 2;
    }
}

fn main() {
    let n = 7;

    let result = Solution::sum_of_multiples(n);
    println!("{}", result);
}