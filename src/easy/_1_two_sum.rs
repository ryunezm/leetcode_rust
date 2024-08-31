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
-> 2 <= nums.length <= 10^4
-> -10^9 <= nums[i] <= 10^9
-> -10^9 <= target <= 10^9
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
                return vec![a, b];
            }
        }
    }

    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2])
    }

    #[test]
    fn test_two_sum_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1])
    }

    #[test]
    fn test_two_sum_case_4() {
        let nums = vec![0, 2, 3, 5, 1];
        let target = 1;
        assert_eq!(two_sum(nums, target), vec![0, 4])
    }
}
