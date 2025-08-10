const nums = [10, 9, 2, 5, 3, 7, 101, 18]

function LIS (list) {
  if (list.length === 0) {
    return []
  }

  let res = [list[0]]
  for (let i = 1; i <= list.length - 1; i += 1) {
    const num = list[i]

    if (num > res.at(-1)) {
      res.push(num)
    } else {
      let l = 0
      let r = res.length - 1
      while (l < r) {
        const mid = l + (r - l) / 2 | 0

        if (res[mid] == num) {
          break
        }

        if (res[mid] > num) {
          r = mid
        } else {
          l = mid + 1
        }
      }

      res[l] = num
    }
  }

  return res
}

console.log('-->  [ LIS(nums) ]  <--\n', LIS(nums))