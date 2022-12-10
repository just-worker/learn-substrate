#![allow(dead_code)]

/// sum of u32
/// ```rust
/// fn main() {
///     let arr = [1,2,3,4,5,6,u32::MAX];
///     match homework2::sum::sum(&arr) {
///         Ok(s) => println!("sum: {}", s),
///         Err(msg) => println!("sum err: {}", msg)
///     }
///}
/// ```
pub fn sum(arr: &[u32]) -> Result<u32, String> {
    let s: u64 = arr.iter().map(|e| e.to_owned()).map(|e| e as u64).sum();
    if s > u32::MAX as u64 {
        return Err("over max u32".to_string());
    }
    return Ok(s as u32);
}
