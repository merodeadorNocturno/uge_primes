use crate::examples::three::is_prime_32;

pub fn counter (up_to: u64) -> Vec<u64> {
  println!("Start");
  let mut my_vec: Vec<u64> = Vec::new();

  my_vec.push(2);

  for i in 3..up_to {
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

pub fn twitter(my_vec: Vec<u64>) -> Vec<u64> {
  let mut solutions: Vec<u64> = Vec::new();

  let vec_1: Vec<u64> = my_vec.clone();
  let vec_2: Vec<u64> = my_vec.clone();
  let vec_3: Vec<u64> = my_vec.clone();
  let vec_4: Vec<u64> = my_vec.clone();

  for p in &vec_1 {
    for q in &vec_2 {
      for r in &vec_3 {
        for s in &vec_4 {
          if p < q && q < r && r < s {
            if p.pow(2) + q + s == p * q * r {
              let solution: u64 = p.pow(2) * q * s;
              solutions.push(solution)
            }
          }
        }
      }
    }
  }

  solutions
}
