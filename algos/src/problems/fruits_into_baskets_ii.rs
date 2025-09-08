use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut seen = HashSet::new();

        for &fruit in &fruits {
            let mut found = false;
            for (i, &basket) in baskets.iter().enumerate() {
                if seen.contains(&i) {
                    continue;
                }
                if basket >= fruit {
                    found = true;
                    seen.insert(i);
                    break;
                }
            }
            if !found {
                res += 1
            }
        }

        return res;
    }
}
