const fs = require("fs");

const files = fs.readdirSync("./find_words");
const sortedFiles = files.sort((a, b) => (a.length > b.length ? -1 : 1));

for (let i = 0; i < 30; i++) {
  console.log(sortedFiles[i], sortedFiles[i].length, [...Array(sortedFiles[i].length)].map(() => '27').join('*'), '=', Intl.NumberFormat('en', { notation: 'compact' }).format(27**sortedFiles[i].length).replace(/,/g, '.'));
}

console.log('Palavras encontradas: ', files.length);