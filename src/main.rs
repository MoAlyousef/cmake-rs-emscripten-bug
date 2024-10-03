fn main() {
    extern "C" {
        fn my_add(a: i32, b: i32) -> i32;
    }
    let ret = unsafe { my_add(1, 2) };
    println!("{ret}");
}
