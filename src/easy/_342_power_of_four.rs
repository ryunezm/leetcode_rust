/*
342. Power of Four
https://leetcode.com/problems/power-of-four
#Math #Bit_Manipulation #Recursion

Given an integer n, return true if it is a power of four. Otherwise, return false.

An integer n is a power of four, if there exists an integer x such that n == 4^x.


Constraints:
-> -2^31 <= n <= 2^31 - 1


Follow up: Could you solve it without loops/recursion?

*/

pub fn is_power_of_four(n: i32) -> bool {

    if n==1 {
        true
    } else {
        let n_binary: Vec<u32> = format!("{:b}", n)
            .chars()
            .map(|c|c
                .to_digit(10)
                .unwrap())
            .collect();

        if n_binary.len() > 2 //
            && n_binary.len() % 2 != 0
            && n_binary.iter().skip(1).sum::<u32>()==0
        { true } else { false }
    }
}

/*
For my future self and others reading this: powers of 4 have the form
of one [1] followed by an even number of zeros [0]. Example:

4 = 100
16 = 10000
64 = 1000000
256 = 100000000
... and so on.

Therefore, the steps are: binary -> vector -> one and zero-sum check.
*/

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_power_of_four_case_1(){
        let n = 16;
        assert_eq!(is_power_of_four(n), true)
    }

    #[test]
    fn is_power_of_four_case_2(){
        let n = 5;
        assert_eq!(is_power_of_four(n), false)
    }

    #[test]
    fn is_power_of_four_case_3(){
        let n = 1;
        assert_eq!(is_power_of_four(n), true)
    }

    #[test]
    fn is_power_of_four_case_4(){
        let n = 4097;
        assert_eq!(is_power_of_four(n), false)
    }

    #[test]
    fn is_power_of_four_case_5(){
        let n = 4096;
        assert_eq!(is_power_of_four(n), true)
    }

    #[test]
    fn is_power_of_four_case_6(){
        let n = 1048576;
        assert_eq!(is_power_of_four(n), true)
    }

    #[test]
    fn is_power_of_four_case_7(){
        let n = 1048579;
        assert_eq!(is_power_of_four(n), false)
    }

}