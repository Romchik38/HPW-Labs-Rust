fn main() {
    let name = "ser";
    hello(name);
}

fn hello(name: &str) {
    println!("hello {}", name);
}
