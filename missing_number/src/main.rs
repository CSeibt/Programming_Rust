

fn find_missing_number (array_of_interest: &[u32]) -> usize {
    let mut supp_vector = array_of_interest.to_vec();
    for i in 0..supp_vector.len() {
        let size_before = supp_vector.len();
        supp_vector.retain(|&x| x != (i+1).try_into().unwrap());
        if size_before == supp_vector.len() {
            return (i+1).try_into().unwrap();
        }
    }
    return 0;
}


fn main() {
    let a = [6,1,2,9,3,4,7,10,5];
    let missing_nr = find_missing_number(&a);
    println!("The missing number is {}", missing_nr);

}
