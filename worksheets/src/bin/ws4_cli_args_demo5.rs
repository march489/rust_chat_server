fn process<T: Iterator<Item = String>>(args: &mut T) {
    let mut sum = 0;

    while let Some(arg) = args.next() {
        sum += arg.parse::<i32>().unwrap();
    }
    println!("{sum}");
}
fn main() {
    /*
     * Slight change from demo4: now we pass by mutable reference
     */
    let mut args = std::env::args().skip(1);
    process(&mut args); // returns the sum of the integers, but iteration consumes
    process(&mut args); // nothing left, prints 0
}
