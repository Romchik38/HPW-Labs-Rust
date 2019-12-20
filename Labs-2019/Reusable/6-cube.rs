fn main() {
    let number = 10;
    let cube1 = cube(number);
    println!("cube from {} is {}", number, cube1);
}

fn cube(x: i32) -> i32 {
    x * x * x
}
