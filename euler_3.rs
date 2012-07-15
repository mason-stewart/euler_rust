fn main(args: [str]) {
  let whole = 600851475143;
  let mut largest_prime = 1;
  fn is_prime (num :int) -> bool {
    let mut x = 1;
    loop {
      x += 1;
      if num % x == 0 {
        break;
      }
      if x == num - 1 {
        ret true;
      }
    }
    ret false;
  }
  // is_prime(11);

  let mut y = 2;
  let limit = whole/2;
  loop {
    // log(error, is_prime(y));
    if whole % y == 0 && is_prime(y) == true {
      largest_prime = y;
      log(error, "found a new prime factor: "+int::str(largest_prime));
    }
    if y == limit {
      log(error, "largest prime factor is "+int::str(largest_prime));
      break;
    }
    y += 1;
  }
}