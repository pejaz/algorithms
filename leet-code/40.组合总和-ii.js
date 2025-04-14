/*
 * @lc app=leetcode.cn id=40 lang=javascript
 *
 * [40] 组合总和 II
 */

// @lc code=start
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum2 = function(candidates, target) {
  let res = [];
  let path = [];
  let total = 0;
  const len = candidates.length;
  
  const backtracking = (startIndex) => {
      if (total === target) {
          res.push([...path]);
          return;
      }

      const setMap = new Set()

      for (let i = startIndex; i < len; i++) {
          const cur = candidates[i];

          // set去重,不需要回溯
          if (cur > target - total || setMap.has(cur)) continue;
          setMap.add(cur)

          path.push(cur);
          total += cur;

          backtracking(i + 1);

          path.pop();
          total -= cur;
      }
  }
  backtracking(0);
  return res;
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = combinationSum2;
// @after-stub-for-debug-end