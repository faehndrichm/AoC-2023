import * as fs from "fs";

fs.readFile("./input_2.txt", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  data.split("\n").forEach((line) => {
    line;
  });
});
