print = console.log;

const squareArr = (a = []) => {
  const myArr = [];

  for (let i = 0; i < a.length; a++) {
    myArr.push(Math.pow(i, 2));
  }

  return myArr;
}

const arrayMaker = (limit, step) => {
  const myArr = [];
  for (let i = step; i < limit; i += step) {
    myArr.push(i);
  }

  return myArr;
}

const getLimit = () => 101;

const _100 = arrayMaker(getLimit(), 1);
const squaredArr = _100.map(item => Math.pow(item,2));

print(`squared array: ${squaredArr}`);

const squareOfSum = Math.pow(_100.reduce((sum, item) => sum + item, 0), 2);

print(`square of sum: ${squareOfSum}`);

const sumOfSquares = squaredArr.reduce((sum, item) => sum + item, 0);

print(`sum of squares: ${sumOfSquares}`);

let total = squareOfSum - sumOfSquares;

print(`Euler 6: ${total}`);

