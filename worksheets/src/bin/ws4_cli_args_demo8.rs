fn process<T: Iterator<Item = String>>(args: &mut T) {
    let mut result = 0;
    // let cloned_args =
    args.into_iter().for_each(|s| {
        result += s.parse::<i32>().unwrap();
    });

    println!("{result}");
}
fn main() {
    /*
     * Slight change from demo4: now we pass by mutable reference
     */
    let mut args = std::env::args().skip(1);
    let print_thing = || {
        println!("{:?}", args);
    };

    print_thing();
    process(&mut args); // returns the sum of the integers, but iteration consumes
    process(&mut args); // nothing left, prints 0
}

/*
 * TODO: Find a way to make a non-consuming iterator
 */
