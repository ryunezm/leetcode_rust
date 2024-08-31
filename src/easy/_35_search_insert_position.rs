/*
35. Search Insert Position
https://leetcode.com/problems/search-insert-position
#Array #Binary_Search

Given a sorted array of distinct integers and a target value, return the index if the target is
found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Constraints:
-> 1 <= nums.length <= 10^4
-> -10^4 <= nums[i] <= 10^4
-> nums contains distinct values sorted in ascending order.
-> -10^4 <= target <= 10^4

*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.len() - 1 {
        0 => {
            return if target <= nums[0] { 0 } else { 1 }
        }
        1 => {
            if target <= nums[0] { return 0; } else if target > nums[0] && target <= nums[1] { return 1; } else if target > 1 { return 2; }
        }
        2 => {
            if target < nums[0] { return 0; } else if target == nums[0] { return 0; } else if target == nums[1] { return 1; } else if target > nums[0] && target <= nums[1] { return 1; } else if target > nums[1] && target <= nums[2] { return 2; } else if target > nums[2] { return 3; }
        }
        _ => {
            let mut i: i32 = 0;
            let mut j: i32 = (nums.len() - 1) as i32;
            let mut m: i32 = (i + j) / 2;

            loop {
                if target <= nums[i as usize] { return i; } else if target == nums[m as usize] { return m; } else if target == nums[j as usize] { return j; } else if target > nums[j as usize] { return j + 1; } else if target > nums[i as usize] && target < nums[m as usize] {
                    i += 1;
                    j = m;
                    m = (i + j) / 2;
                } else if target > nums[m as usize] && target < nums[j as usize] {
                    i = m + 1;
                    j -= 1;
                    m = (i + j) / 2;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_insert_position_case_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2)
    }

    #[test]
    fn search_insert_position_case_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1)
    }

    #[test]
    fn search_insert_position_case_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4)
    }

    #[test]
    fn search_insert_position_case_4() {
        let nums = vec![2, 3, 5, 6];
        let target = 1;
        assert_eq!(search_insert(nums, target), 0)
    }

    #[test]
    fn search_insert_position_case_5() {
        let nums = vec![2, 3, 5, 6, 43, 50, 99];
        let target = 54;
        assert_eq!(search_insert(nums, target), 6)
    }
}
