fn main() {
  // infinite loops
  let mut x = 1;
  loop {
    println!("{:?}", x);
    x += 1;
    if x == 10 { break; }
  }

  // labelled loops
  'top: for (top_index, top_value) in (0..10).enumerate() {
      'bottom: for (bottom_index, bottom_value) in (10..100).enumerate() {
          if bottom_index % 10 == 0 { continue 'top }
          if top_index % 5 == 0 { continue 'bottom}
      }
  }
}

