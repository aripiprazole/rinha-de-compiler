import * as fs from 'fs';
import { Interpreter } from './interpreter'; // Adjust the path as needed

function getJsonFromFile(path: string): any {
  const fileContent = fs.readFileSync(path, 'utf8');
  return JSON.parse(fileContent);
}

function runTest(filePath: string, expectedString: string): void {
  const jsonValue = getJsonFromFile(filePath);
  const result = new Interpreter(jsonValue).run()['value'];
  const success = result.toString() === expectedString;

  if (success) {
    console.log(`Success running ${filePath} with result ${expectedString}`);
  } else {
    console.log(`Fail running ${filePath} with result ${result} - expected ${expectedString}`);
  }
}

if (require.main === module) {
  runTest('./files/combination.json', '45');
  runTest('./files/fib.json', '55');
  runTest('./files/sum.json', '15');
}
