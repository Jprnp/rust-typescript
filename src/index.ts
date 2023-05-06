const fs = require('fs');

console.log(practice([1, 2, 3], 1));
console.log(practice([1, 2, 3], 3));

function printLinesOfFile(): void {
	let filePath = process.cwd();
	filePath = filePath + '/lines.txt';
	fs.readFileSync(filePath).toString().split('\n').forEach((line: string) => console.log(line));
}

function multiply(value: number | undefined): number | undefined {
	if (!value) {
		return undefined;
	}
	return value * 5;
}

function practice(nums: number[], index: number): number {
	return (nums[index] ?? index) * 5;
}
