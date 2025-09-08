pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();

        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);

            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid
            }
        }

        return lo as i32;
    }
}
