fn main() {
    let a:i32 = 2;
    let b:i32 = 3;
    //same inputs = 0 and //different inputs = 1 
    let mut result: i32;

    result = a ^ b;

    println!("(a ^ b) => {}", result);
}
