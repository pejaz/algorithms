/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        // i 即为快指针，slow 代表新数组的下标
        for i in 0..nums.len(){
            if nums[i] != val {
                nums.swap(slow, i);
                slow += 1;
            }
        }

        if slow == 0 {
            nums.clear();
        }

        return slow as i32;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![1];
        Solution::remove_element(&mut nums, 1);
        assert_eq!(nums, vec![]);

        let mut nums2 = vec![2];
        let res = Solution::remove_element(&mut nums2, 3);
        assert_eq!(nums2, vec![2]);
        assert_eq!(res, 1);
    }
}
