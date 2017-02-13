
fn add(x: i32, y: i32) -> i32 {
    x + y
}


fn main() {

    let x: i32 = 10;
    let y: i32 = 20;

    let f: fn(i32, i32) -> i32 = add;
    let nf = add;
    println!("SUM = {}", nf(30, 30));
    println!("Here is a sum {}", f(x, y));
    println!("Here is another sum {}", add(3, 4));
}
