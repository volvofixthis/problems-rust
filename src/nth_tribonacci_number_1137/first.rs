use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn dp(n: i32, m: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        return Self::dp(n - 3, m) + Self::dp(n - 2, m) + Self::dp(n - 1, m);
    }

    pub fn tribonacci(n: i32) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        return Self::dp(n, &mut m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let i1 = 4;
        let o1 = 4;
        let result = Solution::tribonacci(i1);
        assert_eq!(result, o1);
    }
}
