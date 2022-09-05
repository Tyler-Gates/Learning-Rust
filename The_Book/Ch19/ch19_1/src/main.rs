


//In rust global variables are called static variables
//the names are spelt in SCREAMING_SNAKE_CASE by convention
//similar to constants, but they have a fixed address in memory
static HELLO_WORLD: &str = "Hello, world!";
//they can also be mutable, but must be within unsafe blocks when modifying or accessing
static mut COUNTER: u32 = 0;


fn main() {
    let mut num = 5;
    //the asterik isn't a dereferencer, it's part of the type name
    let r1 = &num as *const i32;//   *const = immutable raw pointer
    let r2 = &mut num as *mut i32;// *mut = muttable raw pointer
    // we can create these raw pointers outside of unsafe blocks
    // we cannot dereference and read the data the raw pointers 
    // are pointing to outside of an unsafe block


    //example that we can't guarantee (make an assumption) that what the 
    //pointer points to is valid
    let address = 0x012345usize;
    let r = address as *const i32;


    //we can read by dereferencing the pointer in unsafe code blocks
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();
    }



    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    //split at mut uses unsafe code blocks within it
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

//this function can only be used inside unsafe code blocks
unsafe fn dangerous() {}


//extern keywordcreates and uses a Foreign Function Interface
//the "C" part defines which application Binary Interface (ABI) will be used
extern "C" {
    fn abs(input: i32) -> i32;
}

//this syntax is for calling rust functions from other languages
//here we can call this function from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//can use unsafe to implement an unsafe trait
//a trait is unsafe when at least one of its methods has some invariant that the compiler can't verify
unsafe trait Foo {
    // methods go here
}
unsafe impl Foo for i32 {
    // method implementations go here
}