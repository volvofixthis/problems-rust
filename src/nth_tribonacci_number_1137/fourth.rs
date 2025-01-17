extern crate test;

struct Solution {}

impl Solution {
    pub fn dp(n: i32, m: &mut Vec<i32>) -> i32 {
        let i: usize = n as usize;
        if m[i] >= 0 {
            return m[i];
        }
        let v = Self::dp(n - 1, m) + Self::dp(n - 2, m) + Self::dp(n - 3, m);
        m[i] = v;
        v
    }

    pub fn tribonacci(n: i32) -> i32 {
        let mut m: Vec<i32> = vec![-1i32; n as usize + 1];
        m[0] = 0;
        m[1] = 1;
        m[2] = 1;
        Self::dp(n, &mut m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn validate() {
        let i1: i32 = 4;
        let o1 = 4;
        let result = Solution::tribonacci(i1);
        assert_eq!(result, o1);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| Solution::tribonacci(20i32));
    }
}
