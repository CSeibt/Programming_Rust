
fn get_subarray<S>(my_array: &[u32], lenght: usize, sum: u32) -> [isize; 2] {
    let mut leftindex: usize = 0;
    let mut rightindex: usize = 1;
    let mut subarray: &[u32] = &my_array[leftindex..rightindex];
    loop {
        if rightindex <= lenght || leftindex < rightindex{
            if subarray.iter().sum::<S>() < sum {
                rightindex = rightindex+1;
            }
            else if subarray.iter().sum() > sum {
                leftindex = leftindex+1;
            }
            else {
                return [leftindex.try_into().unwrap(), rightindex.try_into().unwrap()];
            }
        }
        else {
            return [-1, -1];
        }
        
    }
}


fn main() {
    const n: usize = 6;
    let array: [u32; n] = [1, 3, 2, 5, 9, 34];
    let sum: u32 = 7;
    let arrayslice = get_subarray<S>(&array, n, sum);
    
    println!("The range is {} to {}", arrayslice[0], arrayslice[1])
}
