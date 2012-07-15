fn main() {
  let mut sum = 0;
  let mut x = 999;
  loop {
    if x % 5 == 0 || x % 3 == 0 {
      log(error,"matched on "+int::str(x));
      sum += x;
    }
    x = x -1;
    if x <= 0 { break; }
  }
  io::print("the sum is "+int::str(sum)+"!");
}