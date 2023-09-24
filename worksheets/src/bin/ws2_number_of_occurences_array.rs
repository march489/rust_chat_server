fn find_num_occurrence_array(n: u8, arr: [u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array "arr".
    arr.into_iter()
        .filter(|&x| x == n)
        .collect::<Vec<u8>>()
        .len()
}

//// FOR DEBUGGING
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main() {
    let array = [4, 5, 6, 7, 8, 9, 5, 5, 6, 10];
    // TODO: call find_num_occurrence_array in a loop
    // (with every from 0 to 9 inclusive).
    for i in 0..10 {
        println!(
            "occurences of i = {}: {}",
            i,
            find_num_occurrence_array(i, array)
        );
    }
}
