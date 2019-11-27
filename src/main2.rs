// use std::thread;
use std::time::Instant;

mod examples;
// use examples::two::fibonacci;
// use examples::three::{get_factors, get_primes, is_prime};
// use examples::five::{compare_vectors, iterative, vector_maker};
// use examples::six::{square_vec, vector_maker_64};
// use examples::eight::{greatest_product, make_vector};
// use examples::nine::pythagorean;

use examples::ten::{counter, sum_primes, twitter};

// primes

fn main() {
    // fibonacci

    // let _my_vec: Vec<u64> = fibonacci(34);
    // let _my_even_vector: Vec<u64> = _my_vec.into_iter().filter(|a_vec| a_vec % 2 == 0).collect();
    // let _my_result = &_my_even_vector.iter().fold(0, |mut sum, x| {
    //     sum = sum + x;
    //     sum
    // });

    // const EULER_3: u64 = 600851475143;
    // const EULER_3: u64 = 6008;

    // println!("Hello, world! {:?}", &fibonacci(33));
    // println!("Hello, world! {:?}", _my_result);

    // Euler

    // let my_factor_vec = get_factors(EULER_3);
    // println!("My factors {:?}", get_factors(EULER_3));

    // let mut my_primes_vec = Vec::new();

    // for i in my_factor_vec {
    //     if i == EULER_3 {
    //         continue;
    //     }
    //     if is_prime(i) {
    //         my_primes_vec.push(i);
    //     }
    // }

    // println!("6857 {:?}", &my_primes_vec);

    // FACTORS

    // let _iterative_thread = thread::spawn(move || {
    //     const UPPER_LIMIT: u32 = 3000000;

    //     let _result: Vec<u32> = iterative(UPPER_LIMIT);
    //     // println!("*********** RESULT upperLimit {:?}", &_result);
    // });

    // SUM OF SQUARES

    // const EULER_6: u64 = 101;

    // let _100: Vec<u64> = vector_maker_64(EULER_6, 1);

    // let squared_vec: Vec<u64> = square_vec(&_100);

    // // println!("squared_vec: {:?}", &squared_vec);

    // let mut square_of_sum: u64 = _100.iter().fold(0, |mut sum, x| {
    //     sum = sum + x;
    //     sum
    // });

    // println!("square of sum: {:?}", &square_of_sum);

    // square_of_sum = square_of_sum.pow(2);

    // println!("square of sum 2: {:?}", &square_of_sum);

    // let sum_of_squares: u64 = squared_vec.iter().fold(0, |mut sum, x| {
    //     sum = sum + x;
    //     sum
    // });

    // println!("sum of squares: {:?}", &sum_of_squares);

    // let _total: u64 = square_of_sum - sum_of_squares;

    // println!("Euler 6: {}", &_total);

    // ADJACENT GREATEST PRODUCT

    // println!("Adjacent");

    // let mut result: &u64 = &0;

    // let _adj_vector: Vec<u64> = make_vector();
    // let my_vec: Vec<u64> = greatest_product(13, _adj_vector);

    // for i in &my_vec {
    //     if i > result {
    //         result = i;
    //     }
    // }

    // println!("RESULT ADJACENT {}", result);

    // println!("**************** PHYTAGOREAN EULER 9 ****************");

    // println!("{:?}", pythagorean());

    // println!("**************** SUM PRIME 2000000 EULER 10 ****************");

    // let _handle = thread::spawn(|| {
      //     let new_prime = Instant::now();
      //     let new_vec: Vec<u64> = get_primes(1000);
      //     let _new_sum_of_primes = sum_primes(&new_vec);
      //     let new_prime_end = Instant::now();
      //     // println!("NEW Addition {}, duration: {:?}",_new_sum_of_primes, new_prime_end.duration_since(new_prime));
      // });

    const N: u64 = 1250;
      
    let prime_init = Instant::now();
    let my_vec: Vec<u64> = counter(N);
    let _sum_of_primes = sum_primes(&my_vec);
    let prime_end = Instant::now();
    println!(
        "Search Number: {} \n Addition {} \n duration: {:?}",
        &N,
        _sum_of_primes,
        prime_end.duration_since(prime_init)
    );
    // _handle.join().unwrap();
    // _iterative_thread.join().unwrap();

    // println!("****************  ****************");
    // @ugesaurio

    let _uge_instant = Instant::now();
    let _uge_primes = counter(10);
    let _twitter_primes = twitter(_uge_primes);
    let _uge_instant_end = Instant::now();

    // println!("UGE duration: {:?}", _uge_instant_end.duration_since(_uge_instant));
}
