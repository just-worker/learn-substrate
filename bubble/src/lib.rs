//! 这里是学习substrate第三课的课堂作业，主要实现一个冒泡排序

pub mod bubble;

/// 这里是一个排序的trait，对传入的`Vec<T>`进行排序。
/// 只要`T`实现了[`Eq`]+[`Ord`]都可以使用该方法进行排序。
/// 具体实现办法交由下层对象进行实现。
pub trait Sorter {
    fn sort<T>(&self, arr: &mut Vec<T>)
    where
        T: Ord + Eq;
}
