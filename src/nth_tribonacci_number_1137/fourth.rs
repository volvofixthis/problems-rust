struct Solution {}

impl Solution {
    pub fn dp(n: i32, m: &mut Vec<i32>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let i: usize = n as usize;
        if m[i] != 0 {
            return m[i];
        }
        let v = Self::dp(n - 3, m) + Self::dp(n - 2, m) + Self::dp(n - 1, m);
        m[i] = v;
        v
    }

    pub fn tribonacci(n: i32) -> i32 {
        let mut m: Vec<i32> = vec![0i32; n as usize + 1];
        Self::dp(n, &mut m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let i1: i32 = 4;
        let o1 = 4;
        let result = Solution::tribonacci(i1);
        assert_eq!(result, o1);
    }
}
