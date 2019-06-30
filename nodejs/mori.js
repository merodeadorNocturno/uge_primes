const mori = require('mori');

const ns = [1, 2, 3];
const as = ["a", "b", "c"];
console.log(mori.interleave(ns, as)); // => (1 "a" 2 "b" 3 "c")

const isPrime = (n) => {
  const isPrime = true;
  const myList = firstStage(n);
  return isPrime;
}

console.time('mori');

const s = mori.sortedSetBy((a, b) => {
  console.log('comparing', a, b);
  return a - b;
}, 5, 4, 3, 2, 1);

const log_2    = n => Math.log(n) / Math.LN2;
const is_pow_2 = n => log_2(n) % 1 === 0;

const fibs = () => {
  const pairs = mori.iterate(pair => {
      const x = pair[0];
      const y = pair[1];
      return [y, x + y];
  }, [0, 1]);
  return mori.map(mori.first, pairs);
}

// What are the first ten values in the Fibonacci sequence?
console.log(
  mori.take(10, fibs())
);

// Outputs:
// > (0 1 1 2 3 5 8 13 21 34)

// What is the 100th number in the Fibonacci sequence?
console.log(
  mori.nth(fibs(), 100)
);

// Outputs:
// > 354224848179262000000

// What is the sum of the first 4 Fibonacci numbers that are also
// powers of 2?
console.log(
  mori.reduce(mori.sum, 0, mori.take(4, mori.filter(is_pow_2, fibs())))
);

// Outputs:
// > 12

// What is the 5th Fibonacci number that is also a power of 2?
console.log(
  // '->', mori.nth(mori.take(4, mori.filter(is_pow_2, fibs())), 4)
);

// My computer runs for a long time with no apparent answer...

const seq_1 = mori.map(e => {
  console.log('first pass:', e);
  return e;
}, s);
const seq_2 = mori.map(e => {
  console.log('second pass:', e);
  return e;
}, seq_1);

console.log(
  mori.take(2, seq_2)
);

console.timeEnd('mori');

const r = mori.range(3, 1000000, 2);