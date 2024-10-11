/*
You are a professional robber planning to rob houses along a street. Each house has a certain amount
of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses
have security systems connected, and it will automatically contact the police if two adjacent houses
were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum
amount of money you can rob tonight without alerting the police.
https://leetcode.com/problems/house-robber/

Constraints:
1 <= nums.length <= 100
0 <= nums[i] <= 400
*/
/*
pub fn rob(nums: Vec<i32>) -> i32 {
    let mut odd = 0;
    let mut even = 0;

    for i in (0..nums.len()).step_by(2) {
        even = even + nums[i];
    }

    for i in (1..nums.len()).step_by(2) {
        odd = odd + nums[i];
    }

    if even>odd { even } else { odd }
}

*/