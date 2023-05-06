const fs = require('fs');

printLinesOfFileFromArguments()

function printLinesOfFile(): void {
	let filePath = process.cwd();
	filePath = 'lines.txt';
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

function printLinesOfFileFromArguments(): void {
	const filePath = process.argv[2];
	if (!filePath) {
		throw new Error("File path not provided as a parameter");
	}
	fs.readFileSync(filePath).toString().split('\n').forEach((line: string) => console.log(line));
}
