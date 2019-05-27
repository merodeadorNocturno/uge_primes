use crate::examples::three::is_prime_32;

pub fn counter () -> Vec<u64> {
  println!("Start");
  let mut my_vec: Vec<u64> = Vec::new();

  for i in 3..2000000 {
    if i % 2 != 0 {
      if is_prime_32(i as u32) {
        my_vec.push(i as u64);
      }
    }
  }
  println!("Finish");
  my_vec
}

pub fn sum_primes (my_vec: Vec<u64>) -> u64 {
  my_vec.iter().sum()
}

// 142913828519