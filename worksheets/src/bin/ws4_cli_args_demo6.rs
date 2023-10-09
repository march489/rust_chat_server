fn process<T: Iterator<Item = String>>(args: &mut T) {
    let result = args
        .by_ref()
        .map(|s| s.clone().parse::<i32>().unwrap())
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("{}", result);
}
fn main() {
    /*
     * Slight change from demo4: now we pass by mutable reference
     */
    let mut args = std::env::args().skip(1);
    process(&mut args); // returns the sum of the integers, but iteration consumes
    process(&mut args); // nothing left, prints 0
}

/*
 * TODO: Find a way to make a non-consuming iterator
 */
