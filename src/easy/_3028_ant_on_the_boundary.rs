/*
3028. Ant on the Boundary
https://leetcode.com/contest/weekly-contest-383/problems/ant-on-the-boundary
https://leetcode.com/problems/ant-on-the-boundary


An ant is on a boundary. It sometimes goes left and sometimes right.

You are given an array of non-zero integers nums. The ant starts reading nums from the first
element of it to its end. At each step, it moves according to the value of the current element:

-> If nums[i] < 0, it moves left by -nums[i] units.
-> If nums[i] > 0, it moves right by nums[i] units.

Return the number of times the ant returns to the boundary.


Notes:
-> There is an infinite space on both sides of the boundary.
-> We check whether the ant is on the boundary only after it has moved |nums[i]| units.
In other words, if the ant crosses the boundary during its movement, it does not count.


Constraints:
-> 1 <= nums.length <= 100
-> -10 <= nums[i] <= 10
-> nums[i] != 0

*/

pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut bound_count = 0;
    let mut bound_acum = nums[0];

    for i in 1..nums.len() {
        bound_acum = bound_acum + nums[i];

        if bound_acum==0 { bound_count+=1; }
    }

    bound_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_to_boundary_count() {
        // Basic case where there is no return to zero
        let nums1 = vec![1, 2, 3, 4];
        assert_eq!(return_to_boundary_count(nums1), 0);

        // Case where there is a return to zero
        let nums2 = vec![1, -1, 2, -2];
        assert_eq!(return_to_boundary_count(nums2), 2);

        // Case where the vector has only one element
        let nums3 = vec![5];
        assert_eq!(return_to_boundary_count(nums3), 0);

        // Case with multiple returns to zero
        let nums4 = vec![1, -1, 1, -1, 2, -2];
        assert_eq!(return_to_boundary_count(nums4), 3);

        // Case with values that add up to zero but do not return to zero after the first element
        let nums5 = vec![0, 0, 0, 0];
        assert_eq!(return_to_boundary_count(nums5), 3);
    }
}