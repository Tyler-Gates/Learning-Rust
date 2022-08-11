use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");//to see backtrace
    //panic!("crash and burn"); causes error and exit with output crash and burn
    let v = vec![0,1,2,3,4];
    v[99];
}
