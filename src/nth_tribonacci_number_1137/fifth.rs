extern crate test;

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        if n == 0 {
            return a;
        }
        if n == 1 {
            return b;
        }
        if n == 2 {
            return c;
        }
        for _ in 2..n {
            let v = a + b + c;
            a = b;
            b = c;
            c = v;
        }
        c
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
