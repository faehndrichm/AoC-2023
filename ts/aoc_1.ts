import * as fs from "fs";

fs.readFile("./input_1.txt", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  const sum = data
    .split("\n")
    .map((line) => {
      const regex = new RegExp(/one|two|three|four|five|six|seven|eight|nine/g);
      const replacer = (s: string) =>
        ({
          one: "o1e",
          two: "t2o",
          three: "t3e",
          four: "f4r",
          five: "f5e",
          six: "s6x",
          seven: "s7n",
          eight: "e8t",
          nine: "n9e",
        }[s] || "");
      let replline = line.replace(regex, replacer);
      replline = replline.replace(regex, replacer);
      console.log(line);
      console.log(replline);
      let matches = Array.from(replline.matchAll(new RegExp(/[1-9]/g)));
      if (matches?.length) {
        const num = Number(`${matches[0]}${matches[matches.length - 1]}`);

        console.log(num);
        return num;
      } else {
        console.error("oh oh");
        return 0;
      }
    })
    .reduce((sum, x) => sum + x, 0);
  console.log(data.split("\n").length);

  console.log(sum);
});
