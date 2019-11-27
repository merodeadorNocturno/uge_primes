// Primes
#[allow(dead_code)]

pub fn is_prime(my_number: u64) -> bool {
  let mut is_my_number_prime: bool = true;
  if my_number % 2 == 0 {
    is_my_number_prime = false;
  } else if my_number == 1 {
    is_my_number_prime = false;
  } else if my_number == 2 {
    is_my_number_prime = false;
  }

  for i in (3..my_number).rev() {
    if my_number % i == 0 {
      is_my_number_prime = false;
    }
  }

  is_my_number_prime
}

pub fn is_prime_32(my_number: u32) -> bool {
  let mut is_my_number_prime: bool = true;

  if my_number % 2 == 0 {
    is_my_number_prime = false;
  } else if my_number > 5 && my_number % 5 == 0 {
    is_my_number_prime = false;
  } else if my_number > 3 && my_number % 3 == 0 {
    is_my_number_prime = false;
  } else if my_number > 7 && my_number % 7 == 0 {
    is_my_number_prime = false;
  } else if my_number > 11 && my_number % 11 == 0 {
    is_my_number_prime = false;
  } else if my_number > 13 && my_number % 13 == 0 {
    is_my_number_prime = false;
  } else if my_number > 17 && my_number % 17 == 0 {
    is_my_number_prime = false;
  } else if my_number > 19 && my_number % 19 == 0 {
    is_my_number_prime = false;
  } else if my_number > 23 && my_number % 23 == 0 {
    is_my_number_prime = false;
  } else if my_number > 29 && my_number % 29 == 0 {
    is_my_number_prime = false;
  } else if my_number > 31 && my_number % 31 == 0 {
    is_my_number_prime = false;
  } else if my_number > 37 && my_number % 37 == 0 {
    is_my_number_prime = false;
  } else if my_number > 41 && my_number % 41 == 0 {
    is_my_number_prime = false;
  }

  if is_my_number_prime == true {
    for i in (3..my_number).rev() {
      if my_number % i == 0 {
        is_my_number_prime = false;
      }
    }
  }

  is_my_number_prime
}

#[allow(dead_code)]
pub fn get_primes(n: u64) -> Vec<u64> {
  // println!("THREE Start");

  let mut p: u64 = 2;

  let mut list: Vec<u64> = Vec::new();
  let mut primes: Vec<u64> = Vec::new();
  // let mut done: bool = false;

  primes.push(2);

  while p < n {
    let mut i: u64 = 1;

    while p * i <= n {
      // println!("One: {}, {}, {}", &p, &i, &p * i);
      list.push(p * i);
      i += 1;
    }

    while p < n {
      // println!("Two: {}, {}, {}", &p, &i, &p * i);
      p += 1;
      if !list.contains(&p) {
        primes.push(p);
        break;
      }
    }
  }
  // println!("THREE FINISH");
  primes
}

#[allow(dead_code)]
pub fn get_factors(n: u64) -> Vec<u64> {
  let mut my_vec = Vec::new();
  let sqr_number = (n as f64).sqrt() as u64;
  for i in 1..sqr_number {
    if n % i == 0 {
      my_vec.push(i);
      if i != sqr_number {
        my_vec.push(n / i);
      }
    }
  }
  my_vec
}
