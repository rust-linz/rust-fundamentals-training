const board = document.getElementById('board');
const initSection = document.getElementById('initGame');
const gameSection = document.getElementById('game');
const playerName = document.getElementById('playerName');
const welcomePlayer = document.getElementById('welcomePlayer');
let started = false;

let gameId;

for(let row = 0; row < 10; row++) {
    const newRow = board.insertRow(-1);
    for(let col = 0; col < 10; col++) {
        const newCell = newRow.insertCell(-1);
        newCell.dataset.loc = `${ String.fromCharCode('A'.charCodeAt(0) + col) }${row + 1}`;
        newCell.addEventListener("click", elem => shoot(elem.target.dataset.loc));
    }    
}

async function startGame() {
    if (playerName.value) {
        welcomePlayer.innerText = `Ok, ${playerName.value}, let's play 🦜🏴‍☠️!`;

        initSection.hidden = true;
        gameSection.hidden = false;

        const response = await fetch('/games', {
            method: 'POST', 
            headers: { 'Content-Type': 'application/json' }, 
            body: JSON.stringify({ player: playerName.value})
        });

        gameId = await response.json();
    }
}

async function shoot(location) {
    const response = await fetch(`/games/${gameId}/shoot`, {
        method: 'POST', 
        headers: { 'Content-Type': 'application/json' }, 
        body: JSON.stringify(location)
    });

    const responseObj = await response.json();
    const boardContent = responseObj.board;
    let ix = 0;
    for(const tr of board.lastElementChild.children) {
        for(const td of tr.children) {
            switch (boardContent[ix++]) {
                case '~':
                    td.className = 'water'
                    break;
                case 'h':
                    td.className = 'hit'
                    break;
                case 'X':
                    td.className = 'sunken'
                    break;
                default:
                    break;
            }
        }
    }

    switch (responseObj.game_status) {
        case 1:
            welcomePlayer.innerText = `Congrats, ${playerName.value}, you won 🎉🥳!`;
            break;
        case 2:
            welcomePlayer.innerText = `Sorry, ${playerName.value}, too many shots 😢!`;
            break;
        default:
            break;
    }
}