import * as fs from 'fs';
import { Interpreter } from './interpreter';

function main() {
  const args = process.argv.slice(2);

  if (args.length !== 1) {
    console.error('Usage: node main.js <json_path>');
    process.exit(1);
  }

  const jsonPath = args[0];

    try {
        const decodedAst = readAst(jsonPath);
        const interpreter = new Interpreter(decodedAst);
        interpreter.run();
    }
    catch (e) {
        console.log(e);
    }
}

function readAst (path: string) {
    return JSON.parse(fs.readFileSync(path) as unknown as string);
}

if (require.main === module) {
  main();
}
