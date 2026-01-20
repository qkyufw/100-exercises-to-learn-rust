// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // 获取向量长度
    let len = v.len();

    // 处理边界情况：空向量直接返回0
    if len == 0 {
        return 0;
    }

    // 计算中点位置，将向量分成两半
    let mid = len / 2;

    // 将原向量分成两个新的向量（根据hint提示，需要分配新向量）
    let first_half: Vec<i32> = v[..mid].to_vec();
    let second_half: Vec<i32> = v[mid..].to_vec();

    // 在第一个线程中计算前半部分的和
    // move 关键字将 first_half 的所有权转移给闭包
    let handle1 = thread::spawn(move || {
        first_half.iter().sum::<i32>()
    });

    // 在第二个线程中计算后半部分的和
    // move 关键字将 second_half 的所有权转移给闭包
    let handle2 = thread::spawn(move || {
        second_half.iter().sum::<i32>()
    });

    // 等待两个线程完成并获取结果
    // join() 返回 Result<i32, Box<dyn Any>>
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();

    // 返回两个部分的和
    sum1 + sum2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
