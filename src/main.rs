use trace::trace;

#[trace]
fn test() {
    println!("Hello, world!");
}

fn main() {
    test()
}
