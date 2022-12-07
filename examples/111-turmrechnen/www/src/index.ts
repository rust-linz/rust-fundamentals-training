import "./index.css";

import * as turm from "turmrechnen";
import { memory } from "turmrechnen/turmrechnen_bg.wasm";
import $ from "cash-dom";

$(() => {
  turm.init();
  $("#calculate").on("click", () => {
    const height = parseInt(<string>$("#height").val());
    const tower = turm.Tower.new(parseInt(<string>$("#basis").val()), height);

    try {
      const values = new Int32Array(
        memory.buffer,
        tower.tower(),
        (height - 1) * 2 * 3
      );
      let ix = 0;
      let resultTable = $("#result-table");
      for (let i = 0; i < height - 1; i++) {
        resultTable.append(
          buildTableRow(values[ix++], "*", values[ix++], values[ix++])
        );
      }
      for (let i = 0; i < height - 1; i++) {
        resultTable.append(
          buildTableRow(values[ix++], "/", values[ix++], values[ix++])
        );
      }
    } finally {
      tower.free();
    }
  });
});

function buildTableRow(
  basis: number,
  op: string,
  param: number,
  result: number
) {
  return `<tr><td>${basis}</td><td>${op}</td><td>${param}</td><td>=</td><td>${result}</td>`;
}
