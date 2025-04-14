var subsetsWithDup = function (nums) {
  const res = []
  const dfs = (index, list) => {
    res.push(list.slice())
    for (i = index; i < nums.length; i++) {
      const popFlag = list.includes(nums[i])
      list.push(nums[i])
      dfs(i + 1, list)
      if (!popFlag) {
        list.pop()
      }
    }
  }
  dfs(0, [])
  return res
}

subsetsWithDup([1, 2, 3])

//传值和传址区别
function edit (a) {
  a = 0
}
let a = 1
edit(a)
console.log(a) //1

function edit1 (a) {
  a[0] = 0
}
let a = [1]
edit(a)
console.log(a[0]) //0

const c = 1,
  d = 2
console.log(d)

console.log(-5 >> 1) //-3
console.log(5 >> 1) //2, 相当于5/2|0
console.log((5 / 2) | 0)
console.log((1 + (5 - 1)) >> 1) //2， 注意：先运行1+4 再>>1
console.log((1 + (5 - 1) / 2) | 0) //3 注意： |0 也是最后运行

const arr = new Array(4).fill(0).map(() => new Array(4).fill(0))
console.log(arr)

function test (a, ...args) {
  console.log(args);
  function F () {
    console.log('11');
  }
  F()
}
test(1)

async function sleepFunc (time) {
  await sleep(time)
  console.log('I wake up');
}
// 休眠函数
// @param {number} n 毫秒
function sleep (n) {
  let start = new Date().getTime();
  console.log(start);
  while (true) {
    if (new Date().getTime() - start > n) {
      break;
    }
  }
}

function sleep (n) {
  let start = new Date().getTime();
  console.log(start);
  while (true) {
    if (new Date().getTime() - start > n) {
      // 使用  break  实现；
      break;
    }
  }
}
sleepFunc(2000)