/*
2469. Convert the Temperature
https://leetcode.com/problems/convert-the-temperature
#Math

You are given a non-negative floating point number rounded to two decimal
places Celsius, that denotes the temperature in Celsius.

You should convert Celsius into Kelvin and Fahrenheit and return it as an
array ans = [Kelvin, Fahrenheit].

Return the array ans. Answers within 10^-5 of the actual answer will be accepted.

Note that:
-> Kelvin = Celsius + 273.15
-> Fahrenheit = Celsius * 1.80 + 32.00


Constraints:
-> 0 <= celsius <= 1000

*/

pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    Vec::from([celsius + 273.15, celsius * 1.80 + 32.00])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_temperature_test_1() {
        let _celsius: f64 = 36.50;
        let result = Vec::from([309.65000, 97.70000]);
        assert_eq!(convert_temperature(_celsius), result);
    }

    #[test]
    fn convert_temperature_test_2() {
        let _celsius: f64 = 122.11;
        let result = Vec::from([395.26000, 251.79800]);
        assert_eq!(convert_temperature(_celsius), result);
    }
}
