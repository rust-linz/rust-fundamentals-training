import { COLS, ROWS } from './constants';
import { Result } from './result';

export class Board {
  private currentRow = 0;
  private currentCol = 0;
  private isGameOver = false;
  private output: HTMLDivElement[][];
  private errorArea: HTMLDivElement;

  constructor(
    private container: HTMLDivElement,
    private result: number,
    private onRowComplete: (input: string, numberOfGuesses: number) => Result
  ) {
    this.initialize();
  }

  private initialize() {
    this.output = new Array<HTMLDivElement[]>(ROWS);
    for (let row = 0; row < ROWS; row++) {
      this.output[row] = new Array<HTMLDivElement>(COLS);
    }

    // header
    const header = document.createElement('div');
    header.id = 'header';
    header.innerText = `Enter a formula resulting in ${this.result}`;
    this.container.appendChild(header);

    // output
    const outputArea = document.createElement('div');
    outputArea.id = 'output-area';
    this.container.appendChild(outputArea);

    for (let row = 0; row < ROWS; row++) {
      for (let col = 0; col < COLS; col++) {
        const outputField = document.createElement('div');
        outputField.classList.add('output-field');
        outputArea.appendChild(outputField);
        this.output[row][col] = outputField;
      }
    }

    // keyboard
    const keyboardArea = document.createElement('div');
    keyboardArea.id = 'keyboard-area';
    this.container.appendChild(keyboardArea);

    for (let n = 0; n < 10; n++) {
      const inputField = document.createElement('div');
      inputField.onclick = (event) => this.addInput(n.toString());
      inputField.classList.add('key');
      inputField.innerText = n.toString();
      keyboardArea.appendChild(inputField);
    }

    const keys = ['+', '-', '*', '/', 'Enter', 'Delete'];
    for (let i = 0; i < keys.length; i++) {
      const key = document.createElement('div');
      key.onclick = (event) => this.addInput(keys[i]);
      key.classList.add('key');

      if (keys[i].length > 1) {
        key.classList.add(keys[i].toLowerCase());
      }

      key.innerText = keys[i].toString();
      keyboardArea.appendChild(key);
    }

    document.addEventListener('keyup', (event: KeyboardEvent) => {
      const key = event.key === 'Backspace' ? 'Delete' : event.key;
      if ((key >= '0' && key <= '9') || keys.includes(key)) {
        this.addInput(key);
      }
    });

    // error
    this.errorArea = <HTMLDivElement>document.createElement('div');
    this.errorArea.id = 'error-area';
    this.container.appendChild(this.errorArea);
  }

  private addInput(input: string): void {
    if (this.isGameOver) {
      return;
    }

    this.errorArea.innerText = '';

    if (input === 'Delete') {
      if (this.currentCol > 0) {
        this.currentCol--;
        this.output[this.currentRow][this.currentCol].innerText = '';
      }
    } else if (input === 'Enter') {
      if (this.currentCol === COLS) {
        const result = this.onRowComplete(
          this.output[this.currentRow].map((div) => div.innerText).join(''),
          this.currentRow
        );

        if (result.error) {
          this.errorArea.innerText = result.error;
        } else {
          for (let col = 0; col < result.input.length; col++) {
            this.output[this.currentRow][col].style.color = '#fff';

            switch (result.input[col].state) {
              case 'correctSpot':
                this.output[this.currentRow][col].style.backgroundColor =
                  '#24B300';
                break;
              case 'wrongSpot':
                this.output[this.currentRow][col].style.backgroundColor =
                  '#B8A500';
                break;
              case 'notInSolution':
                this.output[this.currentRow][col].style.backgroundColor =
                  '#000';
                break;
            }
          }
        }

        switch (result.status) {
          case 'inProgress':
            if (!result.error) { 
              this.currentRow++; 
              this.currentCol = 0;
            }
            break;
          case 'won':
            this.isGameOver = true;
            setTimeout(() => alert('YIPPIE'));
            break;
          case 'lost':
            this.isGameOver = true;
            setTimeout(() => alert('GAME OVER'));
            break;
        }
      }
    } else if (this.currentCol < COLS) {
      this.output[this.currentRow][this.currentCol].innerText = input;
      this.currentCol++;
    }
  }
}
