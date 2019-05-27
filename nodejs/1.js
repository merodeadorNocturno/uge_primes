// Multiples of 3 and 5

const my_array = Array(999)
  .fill(0)
  .map((item, index) => index + 1)
  .filter(item => item % 3 === 0 || item % 5 === 0)
  .reduce((acc, current) => acc + current, 0);