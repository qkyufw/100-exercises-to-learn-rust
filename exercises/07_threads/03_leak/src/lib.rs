// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // 处理边界情况：空向量直接返回0
    if v.is_empty() {
        return 0;
    }

    // 使用 Vec::leak 将 Vec 的堆分配"泄漏"
    // 这样做会：
    // 1. 消耗 Vec（获取所有权）
    // 2. 返回一个 &'static mut [i32] 切片引用
    // 3. 内存永远不会被 Rust 自动释放（直到程序退出）
    let slice: &'static [i32] = v.leak();

    // 计算中点位置
    let mid = slice.len() / 2;

    // 在第一个线程中计算前半部分的和
    // move 关键字捕获 mid（i32 会复制）
    // slice 是 &'static，满足线程的 'static 生命周期约束
    let handle1 = thread::spawn(move || {
        slice[..mid].iter().sum::<i32>()
    });

    // 在第二个线程中计算后半部分的和
    let handle2 = thread::spawn(move || {
        slice[mid..].iter().sum::<i32>()
    });

    // 等待两个线程完成并获取结果
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();

    // 返回两个部分的和
    result1 + result2
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
