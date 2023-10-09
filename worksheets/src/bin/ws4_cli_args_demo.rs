fn main() {
    let mut args = std::env::args().skip(1);
    let mut sum = 0;

    while let Some(arg) = args.next() {
        sum += arg.parse::<i32>().unwrap();
    }
    println!("{sum}");
}
