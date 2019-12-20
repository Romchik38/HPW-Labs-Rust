fn main() {
    let a1 = average(10, 20);
    println!("average from 10 and 20 is {}", a1);
}

fn average(a: i32, b: i32) -> i32 {
    (a + b) / 2
}
