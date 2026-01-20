// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    // 计算分割点
    let mid = slice.len() / 2;

    // 在第一个线程中计算前半部分的和
    // slice 是 &'static 的，满足 'static 生命周期约束
    // move 关键字转移 mid 的所有权（i32 会复制）
    // slice 仍然是引用，不拥有它，但因为满足 'static 所以安全
    let handle1 = thread::spawn(move || {
        slice[..mid].iter().sum::<i32>() // turbofish 语法，用来指定泛型类型参数
    });

    // 在第二个线程中计算后半部分的和
    let handle2 = thread::spawn(move || {
        slice[mid..].iter().sum::<i32>()
    });

    // 等待两个线程完成并获取结果
    // join() 返回 Result<i32, Box<dyn Any>>
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
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
