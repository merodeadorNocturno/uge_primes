pub fn fibonacci_reccursive(n: i32) -> u64 {
  if n < 0 {
    panic!("{} is negative!", n);
  }
  match n {
    0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
    1 | 2 => 1,
    3 => 2,
    /*
    50    => 12586269025,
    */
    _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
  }
}

pub fn fibonacci(n: i32) -> Vec<u64> {
  let mut my_vec: Vec<u64> = Vec::new();

  if n < 0 {
    panic!("{} is negative!", n);
  } else if n == 0 {
    panic!("zero is not a right argument to fibonacci()!");
  } else if n == 1 {
    return vec![1];
  }

  let mut sum = 0;
  let mut last = 0;
  let mut curr = 1;
  for _i in 1..n {
    sum = last + curr;
    last = curr;
    curr = sum;
    my_vec.push(curr);
  }

  my_vec
}
