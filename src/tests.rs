use crate::two_sum::two_sum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let input = vec![1, 2, 3, 9, 9, 0, 4];
        let target = 19;
        let result = two_sum(input.clone(), target);
        assert_eq!(result, vec![3, 4]);
    }

}