
fn get_subarray(my_array: &[u32], lenght: usize, sum: u32) -> [isize; 2] {
    let mut leftindex: usize = 0;
    let mut rightindex: usize = 1;
    let mut subarray: &[u32];
    loop {
        if rightindex <= lenght && leftindex < rightindex{
            subarray = &my_array[leftindex..rightindex];
            let subarray_sum: u32 = subarray.iter().sum();
            println!("The subarray sum = {}", subarray_sum);
            if subarray_sum < sum {
                rightindex = rightindex+1;
                println!("rightindex = {}", rightindex);
            }
            else if subarray_sum > sum {
                leftindex = leftindex+1;
                println!("leftindex = {}", leftindex);
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
    const N: usize = 6;
    let array: [u32; N] = [1, 3, 2, 29, 9, 34];
    let sum: u32 = 7;
    let arrayslice = get_subarray(&array, N, sum);
    
    println!("The range is {} to {}", arrayslice[0], arrayslice[1])
}
