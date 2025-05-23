/*
 * @lc app=leetcode.cn id=406 lang=rust
 *
 * [406] 根据身高重建队列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 先确定身高，降维，在满足 k的条件
        people.sort_by(|p1, p2| {
            let [h1, k1] = [p1[0], p1[1]];
            let [h2, k2] = [p2[0], p2[1]];

            if h1 == h2 {
                return k1.cmp(&k2);
            } else {
                return h1.cmp(&h2);
            }
        });

        // 这里也可以按身高倒序排序，然后从左到右遍历插入身高小的。
        // 因为身高小的排序在后面，往前插入不会影响原来的身高
        for i in (0..people.len()).rev() {
            let [h, k] = [people[i][0], people[i][1]];

            if k != 0 {
                // 说明前面要等于 h 的人有 k 个。
                // 这里排序了，前面不可能有身高大于 h 的人
                let mut count = 0;
                for j in 0..i {
                    if people[j][0] == h {
                        count += 1;
                    }
                }
                // 差多少人
                let diff = k - count;

                // 往后调整 diff 位
                let ele = people.remove(i);
                people.insert(i + diff as usize, ele);
            }
        }

        return people;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        //assert_eq!(Solution::reconstruct_queue(vec![]),[]);
        assert!(true)
    }
}
