#! node
import { readFileSync } from 'node:fs';

const input = JSON.parse(readFileSync(process.argv[2], 'utf-8'))

function compute(json) {
    // console.log(json)
    if (typeof json === 'string') return console.log(json)
    for (const [k, v] of Object.entries(json)) {
        console.log(`<${k}>`)
        compute(v)
        console.log(`</${k}>`)
    }
}

compute(input)