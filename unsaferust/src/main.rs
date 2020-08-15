fn main() {
    let mut num = 52;
    println!("num is : {}", num);

    let r1 = &mut num as *mut i32;

    println!("<pointer to number is : {:?}", r1);
}
