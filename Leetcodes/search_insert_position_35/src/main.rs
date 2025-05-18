struct Solution;

// impl Solution {
//     pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//         nums.binary_search(&target).unwrap_or_else(|x| x) as i32
//     }
// }

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as usize);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    println!("{}", Solution::search_insert(nums, 100));
}
