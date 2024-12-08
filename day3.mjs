import { readFileSync } from "fs";

let data = readFileSync("inputs/input3.txt").toString();

let pattern = /(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)/g;

let sum = 0;
let dont = false;
for (const match of data.matchAll(pattern)) {
    // console.log(match.slice(0, 6));
    if (match[1] === "mul") {
        // > '4' * '3'
        // 12
        // Insane but okay
        if (!dont) {
            console.log(match[2], "*", match[3]);
            sum += match[2] * match[3];
        }
    } else if (match[4] === "do") {
        console.log("Do");
        dont = false;
    }
    else if (match[5] === "don't") {
        console.log("Don't");
        dont = true;
    }
    

}

console.log(sum);