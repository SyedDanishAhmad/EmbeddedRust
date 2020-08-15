fn main() {
    let mut integer = 26;
    println!("Value of Integer is : {}", integer);
    println!("Reference of Integer : {:p}\n", &integer);
    let mut string = "Hello Unsafe Rust".to_string();
    //println!("Value of string : {}", string);
    //println!("Value of string : {:p}\n", &string);

    let ref1 = &integer as *const i32;
    println!("Address of ref1 : {:p}", ref1);
    unsafe{
        println!("Value of ref1 : {}\n", *ref1);
    }
    // Syntax to create mutable raw pointer
    let ref2 = &mut integer as *mut i32;
    println!("reference of ref2: {:p}", ref2);
    unsafe{
        println!("Value of ref2: {}", *ref2);
    }
    
    
    
    unsafe{
        *ref2 = 77;
        
    }
    println!("Value of Integer is : {}", integer);
    
    
    


}
