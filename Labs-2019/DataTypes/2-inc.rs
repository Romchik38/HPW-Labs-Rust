fn main() {
    let a = 5;
    let b = inc(a);
    println!("Value of b is {}", b);
}

fn inc(n: i32) -> i32 {
    n + 1
}
