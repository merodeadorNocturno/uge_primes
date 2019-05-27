const arrayMaker = (number, step) => {
  let myArray = [];
  for (let i = step; i < number; i += step) {
    myArray.push(i);
  }
  return myArray;
};

const compareArrays = (a1, a2) => {
  const finalArray = [];
  a1.forEach(element1 => 
    a2.forEach(element2 => {
      if (element1 === element2) {
        finalArray.push(element1);
      }
  }));
  return finalArray;
}

const iterative = (n) => {
  const myArray = [];

  for (x = 0; x < n + 1; x++) {
    if (x % 10 === 0)
      if (x % 20 === 0)
        if (x % 19 === 0)
          if (x % 18 === 0)
            if (x % 17 === 0)
              if (x % 16 === 0)
                if (x % 15 === 0)
                  if (x % 14 === 0)
                    if (x % 13 == 0)
                      if (x % 12 === 0)
                        if (x % 11 === 0)
                          if (x % 7 === 0)
                            if (x % 5 === 0)
                              if (x % 3 === 0)
                                myArray.push(x);
  }
  return myArray;
}; 

console.log(iterative(300000000));