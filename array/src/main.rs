fn main() {
  let list = [1,2,3,4,5,6,7];
  let other_list = [5;10]; // initialize with 5 for 10
  println!("{:?}", other_list);
  println!("{:?}", list);
  println!("{:?}", list.len());
  // println!("{:?}", list[10]); // panic here --> this is how segmentation faults are avoided
  let slice = &other_list[..];
  println!("{:?}", slice);
  let other_slice = &other_list[0..4];
  println!("{:?}", other_slice);
}
