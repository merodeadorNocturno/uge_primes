// Euler problem 4: Numeric palindrome 3 digits.

const getThreeDigit = () => 1000;

const calculatePalindrome = (threeDigit) => {
  let palindromeArray = [];
  for (let i = 100; i < threeDigit; i++) {
    for (let j = 100; j < threeDigit; j++) {
      const mult = i * j;
      const tlum = +(mult).toString().split('').reverse().join('');

      if (mult === tlum) {
        palindromeArray.push(tlum);
      }
    }
  }
  return palindromeArray.sort();
}

console.log(Math.max(...calculatePalindrome(getThreeDigit())));
