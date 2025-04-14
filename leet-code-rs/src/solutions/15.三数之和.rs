/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 对 nums 从小到大排序
        let mut sort_nums = nums.clone();
        sort_nums.sort_unstable();
        println!("{:?}", sort_nums);

        let len = sort_nums.len();
        let mut res = vec![];

        for i in 0..len - 2 {
            let num = sort_nums[i];
            // let (mut j,mut k) = (i+1, len-1);
            let mut j = i + 1;
            let mut k = len - 1;

            // 跳过重复项
            if i > 0 && sort_nums[i - 1] == num {
                continue;
            }

            // 优化
            if num + sort_nums[j] + sort_nums[j + 1] > 0 {
                continue;
            }

            if num + sort_nums[k] + sort_nums[k - 1] < 0 {
                continue;
            }

            while j < k {
                let sum = sort_nums[j] + sort_nums[k] + num;
                if sum == 0 {
                    res.push(vec![sort_nums[j], sort_nums[k], num]);

                    j += 1;
                    // 跳过重复项
                    while j < k && sort_nums[j] == sort_nums[j - 1] {
                        j += 1;
                    }

                    k -= 1;
                    // 跳过重复项
                    while k > j && sort_nums[k] == sort_nums[k + 1] {
                        k -= 1;
                    }
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        return res;
    }
}
// @lc code=end
