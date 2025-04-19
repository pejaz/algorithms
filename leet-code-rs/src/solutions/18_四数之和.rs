/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        nums.sort_unstable();

        let mut res = vec![];
        let len = nums.len();

        for i in 0..len - 3 {
            let n1 = nums[i];
            // 去重
            if i > 0 && n1 == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                let n2 = nums[j];
                // 去重
                if j > i + 1 && n2 == nums[j - 1] {
                    continue;
                }

                // 开始左右滑动空间
                let mut l = j + 1;
                let mut r = len - 1;

                while l < r {
                    let sum = nums[l] as i64 + nums[r] as i64 + n1 as i64 + n2 as i64;
                    if sum == target as i64 {
                        res.push(vec![n1, n2, nums[l], nums[r]]);

                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }

                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    } else if sum > target as i64 {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                }
            }
        }

        return res;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_sum() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let res = Solution::four_sum(nums, -294967296);
        assert_eq!(res.len(), 0);
        assert!(true)
    }
}
