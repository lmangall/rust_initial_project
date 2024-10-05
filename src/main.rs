fn main() {
    let message = "Hello, World!";

    let x: i32 = 42;
    let pi: f64 = 3.14159;
    let is_rust_cool: bool = true;
    let letter_a: char = 'a';

    fn add(x: i32, y: i32) -> i32{
        //x + y // no semicolon to mean return 
        return x + y;
    }

    let x = 42;//shadowing (redefining a variable)

    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    let mut i = 1;//mutable variable (can be changed)
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }


}
