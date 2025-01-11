use std::collections::HashMap;
extern crate test;

struct Solution {}

impl Solution {
    pub fn dp(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        return Self::dp(n - 3) + Self::dp(n - 2) + Self::dp(n - 1);
    }

    pub fn tribonacci(n: i32) -> i32 {
        return Self::dp(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn validate() {
        let i1 = 4;
        let o1 = 4;
        let result = Solution::tribonacci(i1);
        assert_eq!(result, o1);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| Solution::tribonacci(20i32));
    }
}
