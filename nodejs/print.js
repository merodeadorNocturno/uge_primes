const print = (comments, toPrint) => {
  const js = typeof toPrint === "object"
    ? JSON.stringify(toPrint, null, 2)
    : toPrint; 
  return console.log(`${comments} ${js}`);
}

module.exports = {
  print
};