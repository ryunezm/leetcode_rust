/*
1. Two Sum
https://leetcode.com/problems/two-sum
#Array #Hash_Table


Given an array of integers nums and an integer target, return
indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution,
and you may not use the same element twice.

You can return the answer in any order.

Constraints:
-> 2 <= nums.length <= 104
-> -109 <= nums[i] <= 109
-> -109 <= target <= 109
-> Only one valid answer exists.

*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let b: i32;

    for i in 0..(nums.len() - 1) {
        let mut a = nums[i];

        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                a = i as i32;
                b = j as i32;
                return vec![a, b]
            }
        }
    }

    return vec![-1, -1]
}