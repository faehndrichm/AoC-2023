import fs from "node:fs";

function main() {
  fs.readFile("./input_4.txt", "utf8", (err, data) => {
    if (err) {
      console.error(err);
      return;
    }

    let cards = new Map<number, number>();
    data
      .split("\n")
      .filter((x) => x)
      .forEach((l, c) => {
        const card = l.split(":")[1];
        const nums = card.split("|");
        const winNums = nums[0]
          .split(" ")
          .map((x) => +x)
          .filter((x) => x);
        const drawnums = nums[1]
          .split(" ")
          .map((x) => +x)
          .filter((x) => x);

        let times = (cards.get(c) ?? 0) + 1;
        cards.set(c, times);

        let wins = 0;
        drawnums.forEach((num) => {
          if (winNums.includes(num)) {
            wins++;
          }
        });

        let j = 0;
        while (j < wins) {
          cards.set(c + j + 1, (cards.get(c + j + 1) ?? 0) + times);
          j++;
        }
      });
    //TODO:
    let sum = [...cards.values()].reduce((total, cur) => total + cur, 0);
    console.log(sum);
  });
}

main();
console.log(`Runtime: ${performance.now()} ms`);

//new RegExp(/[^a-zA-Z0-9\.]/g)
