import { readFileSync, writeFileSync } from "fs";

const data = readFileSync("inputs/input14.txt").toString();
const newData = data.replaceAll("p=", "").replaceAll(" v=", ",");

writeFileSync("inputs/input14-processed.txt", newData);


