#[allow(unused)]
fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    // 1. 选择 point
    let point = nums[right];

    // 2. 分区
    // idx 左边元素都小于 point
    let mut idx = left;

    // 从 left 到 right 遍历,不包含 right
    for i in left..right {
        if nums[i] < point {
            // 交换
            nums.swap(idx, i);
            idx += 1;
        }
    }
    nums.swap(idx, right);

    // 3. 递归
    if idx > 0 {
        // 防止 idx == 0时，idx-1 越界 不为；usize
        quick_sort(nums, left, idx - 1);
    }
    quick_sort(nums, idx + 1, right);
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    #[test]
    fn test_quick_sort() {
        let mut nums = vec![3, 2, 1, 5, 4];
        let len = nums.len();
        quick_sort(&mut nums, 0, len - 1);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }
}
