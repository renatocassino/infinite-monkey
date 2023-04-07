const fs = require("fs");

const files = fs.readdirSync("./find_words");
const sortedFiles = files.sort((a, b) => (a.length > b.length ? -1 : 1));

for (let i = 0; i < 10; i++) {
  console.log(sortedFiles[i]);
}
