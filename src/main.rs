
fn say_hello(){
    println!("Hi");
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    say_hello();
    println!("{}", add_numbers(2, 3));
}
