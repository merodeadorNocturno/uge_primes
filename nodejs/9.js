const { print } = require('./print');

const pythagorean = () => {
  const arrResults = [];

  for (let a = 0; a < 500; a ++) {
    for (let b = 0; b < 500; b ++) {
      for (let c = 0; c < 500; c++) {
        if (a < b && b < c) {
          const suma = Math.pow(a, 2) + Math.pow(b, 2);
          const result = Math.pow(c, 2);
          if (suma === result) {
            const mil = a + b + c;
            if (mil === 1000) {
              arrResults.push(a);
              arrResults.push(b);
              arrResults.push(c);
            }
          }
        }
      }
    }
  }
  
  return arrResults;
}

print('My result:', pythagorean());