pub fn square_vec(v: &Vec<u64>) -> Vec<u64> {
  let mut my_vec:Vec<u64> = Vec::new();

  for i in 0..v.len() {
    my_vec.push(v[i].pow(2));
  }

  my_vec
}

pub fn vector_maker_64(limit: u64, step: u64) -> Vec<u64> {
  let mut my_vec = Vec::new();
  for i in (step..limit).step_by(step as usize) {
    my_vec.push(i);
  }
  my_vec
}