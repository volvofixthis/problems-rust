struct Solution{}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let result = Solution::word_break("kus".to_string(), vec!["kus1".to_string(), "kus2".to_string()]);
        assert_eq!(result, vec!["kus1".to_string()]);
    }
}
