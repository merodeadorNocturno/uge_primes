pub fn vector_maker(limit: u32, step: u32) -> Vec<u32> {
  let mut my_vec = Vec::new();
  for i in (step..limit).step_by(step as usize) {
    if i % 2 == 0 {
      my_vec.push(i);
    }
  }
  my_vec
}

pub fn compare_vectors(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
  let mut final_vector = Vec::new();
  for i in 0..v1.len() {
    for j in 0..v2.len() {
      if v1[i] == v2[j] {
        final_vector.push(v1[i]);
      }
    }
  }
  final_vector
}

pub fn iterative(n: u32) -> Vec<u32> {
  let mut my_vec = Vec::new();
  for x in 0..n {
    if x % 10 == 0 {
      if x % 20 == 0 {
        if x % 19 == 0 {
          if x % 18 == 0 {
            if x % 17 == 0 {
              if x % 16 == 0 {
                if x % 15 == 0 {
                  if x % 13 == 0 {
                    if x % 12 == 0 {
                      if x % 11 == 0 {
                        if x % 7 == 0 {
                          if x % 5 == 0 {
                            if x % 3 == 0 {
                              my_vec.push(x);
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  my_vec
}
