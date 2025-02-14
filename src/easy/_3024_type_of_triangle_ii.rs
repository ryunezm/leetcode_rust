/*
3024. Type of Triangle II
https://leetcode.com/contest/weekly-contest-383
https://leetcode.com/problems/type-of-triangle-ii

You are given a 0-indexed integer array nums of size 3 which can form the sides of a triangle.

-> A triangle is called equilateral if it has all sides of equal length.
-> A triangle is called isosceles if it has exactly two sides of equal length.
-> A triangle is called scalene if all its sides are of different lengths.

Return a string representing the type of triangle that can be formed or "none" if it cannot form a triangle.


Constraints:
-> nums.length == 3
-> 1 <= nums[i] <= 100

*/
/*
pub fn triangle_type(nums: Vec<i32>) -> String {
    let mut result = String::from("") ;

    if nums[0]+nums[1]<=nums[2] || nums[0]+nums[2]<=nums[1] || nums[1]+nums[2]<=nums[0] {result = String::from("none")}
    else if nums[0]==nums[1] && nums[1]==nums[2] {result = String::from("equilateral")}
    else if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] { result = String::from("isosceles"); }
    else {result = String::from("scalene");}

    result
}

*/
