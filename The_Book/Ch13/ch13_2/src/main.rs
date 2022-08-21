fn main() {
    let v1 = vec![1, 2, 3];

    //lazy - they dont do anything until methods consume the iterator
    let v1_iter = v1.iter();

    //NOTE: You dont have to declare an iterator here, you could use v1, and the compiler will
    //create an iterator and consume it

    //here it takes ownership and makes it mutable BTS
    for val in v1_iter { 
        println!("Got: {}", val);
    }

    let mut v2 = vec![1, 2, 3];
    //here the iterator needed to be defined as mutable, each consumption of value by .next() changes v2_iter
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));

    //grabs immutable references
    v2.iter();
    //grabs mutable references
    v2.iter_mut();
    //takes ownership
    v2.into_iter();
    
    let v1: Vec<i32> = vec![1, 2, 3];
    //.map() are whats called an iterator adaptor, they adapt the iterator and dont consume it
    //must use collect and assign to variable otherwise they wont do anything because iterators are lazy
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    //You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. 
    //But because all iterators are lazy, you have to call one of the consuming adaptor methods to get 
    //results from calls to iterator adaptors.


}

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        //consuming adapater - ex. sum()
        //sum call consumes v1_iter, can no longer call it
        let total: i32 = v1_iter.sum(); 
        assert_eq!(total, 6);
    }