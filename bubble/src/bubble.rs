//! 这里主要实现了[`Sorter`]接口，使用冒泡排序完成排序工作。
#![allow(unused)]
use super::Sorter;

/// 简单对象，实现冒泡排序
/// ```rust
/// use bubble;
/// fn main() {
///     let bubble_sorter = bubble::Bubble::new();
///     let mut data = vec![3,2,1];
///     bubble_sorter.sort(&mut data);
///     assert_eq!(vec![1,2,3], data);
/// }
/// ```
pub struct Bubble {}

impl Bubble {
    /// 返回对象实例  
    pub fn new() -> Self {
        Self {}
    }
}

impl Sorter for Bubble {
    /// 使用冒泡排序实现[`Sorter`] trait
    fn sort<T>(&self, arr: &mut Vec<T>)
    where
        T: Eq + Ord,
    {
        let len = arr.len();
        // 空元素或者单一元素，无需排序
        if len < 2 {
            return;
        }
        // 定义排序轮次，一般 len-1 次即可完成，为了简化代码，进行 len 次排序
        for epoch in 0..len {
            // 每经过一轮边界递减
            for idx in 1..(len - epoch) {
                if arr[idx - 1] > arr[idx] {
                    arr.swap(idx - 1, idx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let bubble_sorter = Bubble::new();
        let mut data = vec![3, 2, 1];
        bubble_sorter.sort(&mut data);
        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    fn test_order() {
        let bubble_sorter = Bubble::new();
        let mut data = vec![1, 2, 3];
        bubble_sorter.sort(&mut data);
        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    fn test_disorder() {
        let bubble_sorter = Bubble::new();
        let mut data = vec![1, 3, 5, 7, 9, 8, 6, 4, 2, 0];
        bubble_sorter.sort(&mut data);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], data);
    }
}
