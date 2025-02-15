/*
3028. Ant on the Boundary
https://leetcode.com/problems/find-the-sum-of-encrypted-integers

You are given an integer array nums containing positive integers. We define a function encrypt such
that encrypt(x) replaces every digit in x with the largest digit in x. For example,
encrypt(523) = 555 and encrypt(213) = 333.

Return the sum of encrypted elements.

Constraints:
-> 1 <= nums.length <= 50
-> 1 <= nums[i] <= 1000


*/

pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    let mut total: Vec<i32> = vec![];

    for num in nums {
        total.push(generate_encrypted_numbers(
            extract_highest_digit(num),
            count_digits(num),
        ));
    }

    fn count_digits(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        n = n.abs();

        (n as f64).log10().floor() as i32 + 1
    }

    fn extract_highest_digit(n: i32) -> i32 {
        let mut n = n.abs();
        let mut index: usize = 0;
        let mut digits = Vec::new();
        let mut highest_digit: i32 = 0;

        if n <= 9 {
            return n;
        }

        while n > 0 {
            digits.push(n % 10); // Get the last digit

            if digits[index] == 9 {
                highest_digit = 9;
                break;
            }

            if digits[index] >= highest_digit {
                highest_digit = digits[index];
            }

            index += 1;
            n /= 10; // Delete the last digit
        }

        highest_digit
    }

    fn generate_encrypted_numbers(hd: i32, cd: i32) -> i32 {
        let vector_pow10: Vec<i32> = (0..cd).map(|x| 10_i32.pow(x as u32)).collect();
        let vector_encrypted: Vec<i32> = vector_pow10.iter().map(|&x| x * hd).collect();
        let encrypted_number: i32 = vector_encrypted.iter().sum();

        encrypted_number
    }

    total.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let num1 = vec![1, 2, 3];
        assert_eq!(sum_of_encrypted_int(num1), 6);
    }

    #[test]
    fn test_2() {
        let num2 = vec![10, 21, 31];
        assert_eq!(sum_of_encrypted_int(num2), 66);
    }

    #[test]
    fn test_3() {
        let num3 = vec![28, 36, 19];
        assert_eq!(sum_of_encrypted_int(num3), 253);
    }

    #[test]
    fn test_4() {
        let num_4 = vec![34512, 187, 29, 112, 8132];
        assert_eq!(sum_of_encrypted_int(num_4), 65652);
    }
}
