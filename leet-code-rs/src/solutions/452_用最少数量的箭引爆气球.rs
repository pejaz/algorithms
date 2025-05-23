/*
 * @lc app=leetcode.cn id=452 lang=rust
 *
 * [452] 用最少数量的箭引爆气球
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| {
            if a[0] == a[0] {
                return a[1].cmp(&b[1]);
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut shots = vec![];

        for point in points {
            if shots.is_empty() {
                shots.push(point);
            } else {
                // 查找交集
                let shot = shots.pop().unwrap();
                let [s, e] = [shot[0], shot[1]];

                if e >= point[0] {
                    // 存在交集
                    let new_shot = vec![point[0], e];
                    shots.push(new_shot);
                } else {
                    shots.push(shot);
                    shots.push(point);
                }
            }
        }
        return shots.len() as i32;
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
    fn test_find_min_arrow_shots() {
        let res = Solution::find_min_arrow_shots(vec![
            vec![3, 9],
            vec![7, 12],
            vec![3, 8],
            vec![6, 8],
            vec![9, 10],
            vec![2, 9],
            vec![0, 9],
            vec![3, 9],
            vec![0, 6],
            vec![2, 8],
        ]);
        println!("{:?}", res);
        assert!(true);
    }
}
