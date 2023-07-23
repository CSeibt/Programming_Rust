fn segregate(original_array: &[u32], array_size: usize) {
    for i in 0..array_size {
        if original_array[i] == 0 {
            print!("{} ", original_array[i]);
        }
    }
    for i in 0..array_size {
        if original_array[i] == 1 {
            print!("{} ", original_array[i]);
        }
    }
    for i in 0..array_size {
        if original_array[i] == 2 {
            print!("{} ", original_array[i]);
        }
    }
} 


fn main() {
    const N: usize = 7;
    let values: [u32; N] = [1, 0, 2, 1, 2, 0, 0];
    segregate(&values, N);
}
