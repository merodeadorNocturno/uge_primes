pub fn pythagorean() -> Vec<u32> {
  let mut vec_results:Vec<u32> = Vec::new();

  for i in 0..500 {
    let a = i as u32;
    for j in 0..500 {
      let b = j as u32;
      for k in 0..500 {
        let c = k as u32;
        if a < b && b < c {
          let suma = a.pow(2) + b.pow(2);
          let result = c.pow(2);
          if suma == result {
            let mil = a + b + c;
            if mil == 1000 {
              vec_results.push(a);
              vec_results.push(b);
              vec_results.push(c);
            }
          }
        }
      }
    }
  }

  vec_results
}