/*
 * @lc app=leetcode.cn id=127 lang=rust
 *
 * [127] 单词接龙
 */

// @lc code=start

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused)]
impl Solution {
    // 解题思路：
    /**
     * 将单词列表构建成图，转化为从 beginword 到 endword 的最短路径问题，在通过 BFS 搜索图
     */
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        // 用 map 来标识路径，key 代表当前节点，val 代表路径长度
        let mut path = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(begin_word.clone());
        word_set.remove(&begin_word);
        path.insert(begin_word, 1); // 代表起点到begin_word 长度为 1 

        while !queue.is_empty() {
            let word = queue.pop_front().unwrap();
            let chars: Vec<char> = word.chars().collect();
            // 尝试用 26个字母轮番替换单词每一个字节
            for i in 0..chars.len() {
                let mut new_chars = chars.clone();
                for c in 'a'..='z' {
                    new_chars[i] = c;
                    let new_word: String = new_chars.clone().into_iter().collect();

                    // 找到下一条路径
                    if word_set.remove(&new_word) {
                        let len = path.get(&word).unwrap() + 1;
                        path.insert(new_word.clone(), len);

                        // 终点,
                        if new_word.eq(&end_word) {
                            return len;
                        }

                        queue.push_back(new_word);
                    }
                }
            }
        }

        return 0;
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
    fn test_ladder_length() {
        //println!("res is {:?}", Solution::ladder_length(vec![]));
        //assert_eq!(Solution::ladder_length(vec![]),[]);
        assert!(true)
    }
}
