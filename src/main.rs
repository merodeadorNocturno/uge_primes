// use std::thread;
use std::time::Instant;

mod examples;
use examples::ten::{counter, get_solutions, sum_primes};

// primes

fn main() {
  const N: u64 = 1250;
  let prime_init = Instant::now();
  // let my_vec: Vec<u64> = counter(N);
  let _uge_instant = Instant::now();
  let _uge_primes = counter(N);
  let _sum_of_primes = sum_primes(&_uge_primes);
  let _twitter_primes = get_solutions(_uge_primes);
  let _uge_instant_end = Instant::now();
  let prime_end = Instant::now();
  println!(
    "Search Number: {} \n Addition {} \n duration: {:?}",
    &N,
    _sum_of_primes,
    prime_end.duration_since(prime_init)
  );
  println!("Solution {:?}", _twitter_primes);
}
