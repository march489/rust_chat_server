fn process(args: &Vec<String>) {
    let result = args
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("{result}");
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let print_thing = || {
        println!("{:?}", args);
    };

    print_thing();
    process(&args);
    process(&args);
}
