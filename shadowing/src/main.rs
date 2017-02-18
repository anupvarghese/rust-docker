/*

Shadowing

*/
fn main() {
    let x :i32;
    x = 5;
    println!("Value of x, {}", x);
    let x = "I am a string";

    // Here X is immutable and changed to another type in later part of the code, this is shadowing
    println!("Who are you x?, {}", x);

    // We can also make x as mutable and keeping the shadow effect.
    let mut x :i32 = 23;
    println!("Value of x, {}", x);
    x = 24;
    println!("Value of x, {}", x);

}
