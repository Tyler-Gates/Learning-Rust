fn main() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    //strings are utf-8 encoded so anything thats properly encoded is good!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //appending strings
    let mut s = String::from("foo");
    s.push_str("bar");
    //ownership demonstration last line wouldn't work if push_str took ownership, it just takes a reference
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    //push is all about CHARs
    let mut s = String::from("lo");
    s.push('l');

    //concat 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //second value has to be a reference because of the functions parameters. first has to be non-reference
    //uses  deref coercion to convert &String into &str

    //inferior
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //superior
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);


    let s1 = String::from("hello");
    //let h = s1[0];
    //rust strings do NOT support indexing like other languages!


    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");

    //takes bytes like a baws, since each of these whacky characters are 2bytes, s would be Зд
    //You should use ranges to create string slices with caution, because doing so can crash your program.
    let hello = "Здравствуйте";
    let s = &hello[0..4];


    //be explicit in what you want, chars or bytes
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    //Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library
}
