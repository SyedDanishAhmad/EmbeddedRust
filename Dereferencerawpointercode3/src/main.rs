fn main() {
    let address = 0x01234abcdusize;

    //let raw_ptr_const = address as *const u32;
    let raw_ptr_const = address as *mut u32;
    unsafe {
        println!("Vaue of raw_ptr_cnst is : {:?}", *raw_ptr_cnst);
    }
    
}
