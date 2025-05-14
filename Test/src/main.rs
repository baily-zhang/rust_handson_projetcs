pub struct Solution;

//impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         match nums.binary_search(&target) {
//             Ok(v) => return v.try_into().unwrap(),
//             Err(e) => return -1, };
// }
// }

// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         let mut left = 0;
//         let mut right = nums.len() - 1;
//
//         while left <= right {
//             let mid = left + (right - left) / 2;
//             let mid_val = nums[mid];
//
//             if mid_val == target {
//                 return mid as i32;
//             } else if mid_val < target {
//                 left = mid + 1;
//             } else {
//                 right = mid - 1;
//             }
//         }
//         -1
//     }
// }

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] == target {
                return mid as i32;
            } else {
                right = mid;
            }
        }
        -1
    }
}

// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         match nums.binary_search(&target) {
//             Ok(v) => v.try_into().unwrap(),
//             Err(e) => -1,
//         }
//     }
// }
fn main() {
    let res = Solution::search(vec![-5, 2, 4], 4);
    println!("{res}");
    println!("{}", Solution::search(vec![-5, 2, 4], -5));
}
