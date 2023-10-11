use std::cell::Cell;

fn main() {
    let x: Cell<i32> = Cell::new(5);
    let f = |y: i32| -> i32 { x.get() + y };
    // Not allowed because x is borrowed by f
    // x = x + 1;
    x.set(x.get() + 1);
    let v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", Vec::from_iter(v.into_iter().map(f)));
}
