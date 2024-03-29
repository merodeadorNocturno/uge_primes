const isPrime = (myNumber = 7) => {
  let isMyNumberPrime = true;

  if (myNumber % 2 === 0) {
    return false;
  }

  for (let i = 3; i < myNumber; i++) {
    const modulus = myNumber % i;
    if (modulus === 0) {
      isMyNumberPrime = false;
    } 
  } 

  return isMyNumberPrime;
};

const counter = (upTo = 1) => {
  const myArr = [2];

  for (i = 3; i < upTo; i++) {
    if (i % 2 !== 0 && isPrime(i)) {
      myArr.push(i);
    }
  }
  return myArr;
};

const twitter = (myArr = [7]) => simpleForSolution(myArr);

const simpleForSolution = (myArr = [7]) => {
  let solutions = [];

  for (let i = 0; i < myArr.length; i++) {
    let p = myArr[i];
    for (let j = 0; j < myArr.length; j++) {
      let q = myArr[j];
      for (let k = 0; k < myArr.length; k++) {
        let r = myArr[k];
        for (let l = 0; l < myArr.length; l++) {
          let s = myArr[l];
          if (p < q && q < r && r < s) {
            const pSquared = p * p;
            const mySum = pSquared + q + s;
            const myMult = p * q * r;
            if (mySum === myMult) {
              const solution = pSquared * q * s - 1;
              solutions.push(solution);
            }
          }
        }
      }
    }
  }
  return solutions;
}

const forOfSolution = (myArr = [7]) => {
  let solutions = [];

  for (p of myArr) {
    for (q of myArr) {
      for (r of myArr) {
        for (s of myArr) {
          if (p < q && q < r && r < s) {
            const pSquared = p * p;
            const mySum = pSquared + q + s;
            const myMult = p * q * r;
            if (mySum === myMult) {
              const solution = pSquared * q * s - 1;
              solutions.push(solution);
            }
          }
        }
      }
    }
  }
  return solutions;
}

const searchNumber = 1250;
console.time('duration');
const ugePrimes = counter(searchNumber);
const myTwit = twitter(ugePrimes);
const addition = ugePrimes.reduce((acc, item) => acc + item, 0);
console.timeEnd('duration');
console.log('Search Number:', searchNumber);
console.log('Addition:', addition);

// console.log(JSON.stringify(myTwit, null, 2));

