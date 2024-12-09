import { readFileSync } from "fs";

let data = readFileSync("inputs/input8.txt").toString().split("\n");

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

function getAntinodes(a1, a2) {
    const d = [a2[0] - a1[0], a2[1] - a1[1]];
    return [
        [a1[0] - d[0], a1[1] - d[1]],
        [a2[0] + d[0], a2[1] + d[1]]
    ];
}

function* getAllInLine(a1, a2) {
    const d = [a2[0] - a1[0], a2[1] - a1[1]];
    let nextPoint = a1;

    while (true) {
        nextPoint = [nextPoint[0] + d[0], nextPoint[1] + d[1]];
        if (inGrid(nextPoint)) yield nextPoint.toString();
        else break;
    }

    nextPoint = a2;

    while (true) {
        nextPoint = [nextPoint[0] - d[0], nextPoint[1] - d[1]];
        if (inGrid(nextPoint)) yield nextPoint.toString();
        else break;
    }
}

function inGrid(point) {
    return point[0] >= 0 && 
        point[0] < data.length && 
        point[1] >= 0 &&
        point[1] < data[0].length;
}



function part1() {
    const antinodes = new Set();

    for (const group of antennas.values()) {
        for (let i = 0; i < group.length; i++) {
            for (let j = i + 1; j < group.length; j++) {
                const possibleAntinodes = getAntinodes(group[i], group[j]);
                for (const elem of possibleAntinodes) {
                    if (inGrid(elem)) antinodes.add(elem.toString());
                }
            }
        }
    }
    console.log(antinodes.size);
}

function part2() {
    const antinodes = new Set();
    for (const group of antennas.values()) {
        for (let i = 0; i < group.length; i++) {
            for (let j = i + 1; j < group.length; j++) {
                for (const point of getAllInLine(group[i], group[j])) {
                    antinodes.add(point);
                }
            }
        }
    }
    
    console.log(antinodes.size);
}

part2();





