use crate::examples::three::is_prime_32;

pub fn counter (up_to: u64) -> Vec<u64> {
  // println!("Start");
  let mut my_vec: Vec<u64> = Vec::new();

  my_vec.push(2);

  for i in 3..up_to {
    if i % 2 != 0 {
      if is_prime_32(i as u32) {
        my_vec.push(i as u64);
      }
    }
  }
  // println!("Finish");
  my_vec
}

pub fn sum_primes (my_vec: &Vec<u64>) -> u64 {
  my_vec.iter().sum()
}

pub fn get_solutions(my_vec: Vec<u64>) -> Vec<u64> {
  let mut solutions: Vec<u64> = Vec::new();

  let vec_1: Vec<u64> = my_vec.clone();
  // println!("vec_1: {:?}", &my_vec);

  for p in &vec_1 {
   for q in &vec_1 {
     for r in &vec_1 {
       for s in &vec_1 {
          if p < q && q < r && r < s {
            let p_squared = p.pow(2);
            let my_sum = p_squared + q + s;
            let my_mult = p * q * r;
            if my_sum == my_mult {
              if (r * s - 1) == (p * q) + (p.pow(2) * q.pow(2)) + (p.pow(3) * q.pow(3)) {
                println!("rs-1: {}", r * s -1);
                println!("p.pow2 + q + s: {} ... my_mult {}", &my_sum,  &my_mult);
                println!("p: {} q: {} r: {} s: {}", &p, &q, &r, &s);
                let solution: u64 = p_squared * q * s - 1;
                solutions.push(solution)
              }
            }
          }
        }
      }
    }
  }

  solutions
}
