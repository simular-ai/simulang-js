// Run: node examples/file-trait.mjs
// This example writes a file and reads it back.

import { readFile, writeFile } from '@simular-ai/simulang-js'

const writtenPath = writeFile('simulang-js-example.txt', 'hello from simulang-js', false)
console.log('Wrote file:', writtenPath)
console.log('Read file:', readFile(writtenPath))
