fn main() {
    // Formatter :b for binary, :x for hexadecimals
    let mut integer = 26;
    
    println!("Value of integer is : {}", integer);
    println!("Reference of Integer: {}", &integer);
    println!("Reference of Integer: {:p}", &integer);
    println!("Reference of Integer: {:b}", &integer);
    println!("Reference of Integer: {:x}", &integer);
}
