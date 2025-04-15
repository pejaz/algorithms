/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() <= 0 {
            return 0;
        }

        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            // 查找不等于 val 的下标
            while r > l && nums[r] == val {
                r -= 1;
            }
            // 查找等于 val 的下标
            while l <= r && nums[l] != val {
                l += 1
            }

            if l < r {
                nums.swap(l, r);
            }

            if l == r {
                break;
            }
        }

        if l == 0 {
            nums.clear();
        }

        return l as i32;
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
