
fn largest<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main() {
    let integer_list = vec![1, 3, -2, 4059, 939, 5];
    let char_list = vec!['y', 'x', 'z'];
    let float_list = vec![2.23, 44.5, 3.14, 55.595952, 420.69];
    println!("Largest int = {}", largest(&integer_list));
    println!("Largest char = {}", largest(&char_list));
    println!("Largest float = {}", largest(&float_list));

}
