import * as bs from "battleship_wasm";
import { memory } from "battleship_wasm/battleship_wasm_bg.wasm";
import "./index.css";

const game = bs.BattleshipGame.new();
const boardContent = new Uint8Array(memory.buffer, game.board_content(), 100);

const board = document.getElementById('board') as HTMLTableElement;
const won = document.getElementById('won') as HTMLHeadingElement;

for(let row = 0; row < 10; row++) {
    const newRow = board.insertRow(-1);
    for(let col = 0; col < 10; col++) {
        const newCell = newRow.insertCell(-1);
        newCell.dataset.loc = `${ String.fromCharCode('A'.charCodeAt(0) + col) }${row + 1}`;
        newCell.addEventListener("click", elem => shoot((elem.target as HTMLTableElement).dataset.loc));
    }    
}

async function shoot(location: string) {
    const result = game.shoot(location);
    if (result.game_state === "AllShipsSunken") {
        won.hidden = false;
    }

    let ix = 0;
    for(const tr of board.lastElementChild.children) {
        for(const td of tr.children) {
            switch (boardContent[ix++]) {
                case bs.SquareContentJS.Water:
                    td.className = 'water'
                    break;
                case bs.SquareContentJS.HitShip:
                    td.className = 'hit'
                    break;
                case bs.SquareContentJS.SunkenShip:
                    td.className = 'sunken'
                    break;
                default:
                    break;
            }
        }
    }
}
