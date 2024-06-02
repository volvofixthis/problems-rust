struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let mut result = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut result);
        assert_eq!(result, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
