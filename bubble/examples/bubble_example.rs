use bubble::{bubble::Bubble, Sorter};
fn main() {
    let bubble_sorter = Bubble::new();
    let mut data = vec![1, 3, 5, 7, 9, 8, 6, 4, 2, 0];
    bubble_sorter.sort(&mut data);
    println!("sotred data: {:?}", data);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], data);
}
