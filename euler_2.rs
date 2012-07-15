fn main() {
  let mut current = 2;
  let mut prev = 1;
  let mut next = 0;
  let mut sum = current;
  let max = 4000000;
  loop {
    if current > max { break; }
    next = current + prev;
    if next % 2 == 0 && next < max {
      sum += next;
      log(error,"found an even fibonacci number: "+int::str(next));
    }
    prev = current;
    current = next;
  }
  log(error,"Final sum of all even fibonacci numbers below 4,000,000 is "+int::str(sum));
}