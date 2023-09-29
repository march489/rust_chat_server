fn chop_1(s: String) -> Vec<char> {
    ////////////////// Using accumulator ///////////////////
    // let mut v = Vec::<char>::new();

    // for it in s.chars() {
    //     v.push(it);
    // }

    // v

    /////////////// Using collect() //////////////////////////
    s.chars().collect::<Vec<char>>()
}

fn main() {
    let s: String = String::from("Hello, world!");
    println!("{:?}", chop_1(s.clone()));
}
