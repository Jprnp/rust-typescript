const fs = require('fs');

let filePath = process.cwd();
filePath = filePath + '/lines.txt';
fs.readFileSync(filePath).toString().split('\n').forEach((line: string) => console.log(line));