fn process<T: Iterator<Item = String>>(mut args: T) {
    let mut sum = 0;

    while let Some(arg) = args.next() {
        sum += arg.parse::<i32>().unwrap();
    }
    println!("{sum}");
}
fn main() {
    /*
     * We don't need to declare args as MUT because we pass it to process
     * which only accepts mutable parameters
     */
    let args = std::env::args().skip(1);
    process(args);
}
