fn main() {
    let x: i32 = 10;
    let s1: i32 = square(10);
    println!("square from {} is {}", x, s1);
}

fn square(x: i32) -> i32 {
    x * x
}
