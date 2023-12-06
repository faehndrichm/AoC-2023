import fs from "node:fs";
fs.readFile("./input_3.txt", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }

  const matrix = data.split("\n").map((l) => [".", ...l, "."]);

  let sum = 0;
  for (let r = 0; r < matrix.length; r++) {
    for (let c = 0; c < matrix[r].length; c++) {
      if (!isNaN(+matrix[r][c])) {
        // is number
        let cur_num = matrix[r][c];
        let i = 1;
        while (!isNaN(+matrix[r][c + i])) {
          cur_num += matrix[r][c + i];
          i++;
        }
        if (check_adj(c, c + i, r, matrix)) {
          console.log(cur_num);
          sum += +cur_num;
        }
        c += i;
      }
    }
  }
  console.log(sum);
});

function check_adj(
  from: number,
  to: number,
  r: number,
  matrix: string[][]
): boolean {
  for (let i = from - 1; i < to + 1; i++) {
    if (matrix?.[r - 1]?.[i]?.match(new RegExp(/[^a-zA-Z0-9\.]/g))) {
      return true;
    }
    if (matrix?.[r]?.[i]?.match(new RegExp(/[^a-zA-Z0-9\.]/g))) {
      return true;
    }
    if (matrix?.[r + 1]?.[i]?.match(new RegExp(/[^a-zA-Z0-9\.]/g))) {
      return true;
    }
  }
  return false;
}
