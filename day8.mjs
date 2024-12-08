import { readFileSync } from "fs";

let data = readFileSync("inputs/input3.txt").toString().split("\n");

const antennas = new Map();

for (let r = 0; r < data.length; r++) {
    for (let c = 0; c < data[0].length; c++) {
        const sym = data[r][c];
        if (sym != '.') {
            if (antennas.get(sym) === undefined) antennas.set(sym, [[r, c]]);
            else antennas.get(sym).push([r, c]);
        }
    }
}

function getAntinode(a1, a2) {
    const d = [a2[0] - a1[0], a2[1] - a1[1]];
    return [
        [a1[0] - d[0], a1[1] - d[1]],
        [a2[0] + d[0], a2[1] + d[1]]
    ];
}

antinodes = new Set();





