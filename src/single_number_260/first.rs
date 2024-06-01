use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut r: Vec<i32> = vec![];
        for v in nums.iter() {
            match counter.get(&v) {
                Some(_) => {
                    counter.remove(v);
                },
                _ => {
                    counter.insert(*v, 1);
                },
            };
        }
        for (v, _) in counter.into_iter() {
            r.push(v);
        }
        r.sort();
        return r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let result = Solution::single_number(vec![1i32, 2,1,3,2,5]);
        assert_eq!(result, vec![3, 5]);
    }
}
