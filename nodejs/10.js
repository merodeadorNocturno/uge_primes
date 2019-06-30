const isPrime = (myNumber = 7) => {
  let isMyNumberPrime = true;

  if (myNumber % 2 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 3 && myNumber % 3 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 5 && myNumber % 5 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 7 && myNumber % 7 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 11 && myNumber % 11 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 13 && myNumber % 13 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 17 && myNumber % 17 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 19 && myNumber % 19 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 23 && myNumber % 23 === 0) {
    isMyNumberPrime = false;
  } else if (myNumber > 29 && myNumber % 29 === 0) {
    isMyNumberPrime = false;
  }

  if (isMyNumberPrime) {
    for (let i = 3; i < myNumber; i++) {
      if (myNumber % i === 0) {
        isMyNumberPrime = false;
      } 
    } 
  }

  return isMyNumberPrime;
};

const counter = (upTo = 1) => {
  const myArr = [2];

  for (i = 3; i < upTo; i++) {
    if (i % 2 !== 0) {
      if (isPrime(i)) {
        myArr.push(i);
      }
    }
  }
  return myArr;
};

const twitter = (myArr = [7]) => {
  return simpleForSolution(myArr);
}

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

console.time('start');
const ugePrimes = counter(1000);
const myTwit = twitter(ugePrimes);
console.timeEnd('start');

// console.log(JSON.stringify(myTwit, null, 2));