

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    //defining drop to visualize how it works. normally would be code to clean up your type/struct, not println
    //this function is called automatically at the end of a scope where a CustomSmartPointer is created. 
    //you cannot manually call this function
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer c dropped before the end of main.");
//variables are dropped in reverse order of creation, so d, then c.
}


//you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.