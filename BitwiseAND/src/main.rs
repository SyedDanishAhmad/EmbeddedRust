fn main() {
    let a:i32 = 2;  //10
    let b:i32 = 3;  //11
                   //=10
    let mut result: i32;

    result = a & b;
    
    println!("(a & b) => {}", result);
}
