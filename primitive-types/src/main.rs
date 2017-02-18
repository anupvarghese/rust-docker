fn main() {
    // arrays
    let a = [1, 2, 3, 4];
    println!("Array[0] is {}", a[0]);
    let mut b = ['a', 'b'];
    println!("Char array, b[0], {}", b[0]);
    b = ['c', 'd'];
    println!("Char array, b[0] {} and length of b is {}", b[0], b.len());

    // Slices
    let all = &a[..];
    println!("Copied a into all array, all[0] = {} and length = {}", all[0], all.len());

    let last_two = &a[1..3];
    println!("Copied a into all array, all[1] = {}", last_two[1]);

    //tuples
    let x = (1, "hi", 1.234);
    println!("x is a tuple, and x.0 = {} & x.1 = {} & x.2 = {}", x.0, x.1, x.2);

    // destructure tuples
    let ( int, string, float ) = x;
    println!("int = {}, string = {}, float = {}", int, string, float);

}
