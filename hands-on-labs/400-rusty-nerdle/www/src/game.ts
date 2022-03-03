import { generateChallenge } from "nerdle";
import { Board } from "./board";
import { COLS, ROWS } from "./constants";
import { Result, CharState } from "./result";

export class Game {
  private equation: string;
  private expectedResult: number;

  constructor(container: HTMLDivElement) {
    this.createEquation();
    new Board(container, this.expectedResult, (input, numberOfGuesses) =>
      this.evaluateInput(input, numberOfGuesses)
    );
  }

  private evaluateInput(input: string, numberOfGuesses: number): Result {
    const result = new Result();

    let evalResult: number;
    try {
      evalResult = eval(input);
    } catch (error) {
      result.error = "Could not evaluate input.";
    }

    if (evalResult !== undefined) {
      if (evalResult !== this.expectedResult) {
        result.error = "Input is not valid";
      } else {
        const availableChars: string[] = [];

        for (let i = 0; i < COLS; i++) {
          if (this.equation[i] !== input[i]) {
            availableChars.push(this.equation[i]);
          }
        }

        for (let i = 0; i < COLS; i++) {
          let state: CharState = "notInSolution";
          if (this.equation[i] === input[i]) {
            state = "correctSpot";
          } else if (availableChars.includes(input[i])) {
            const index = availableChars.indexOf(input[i]);
            availableChars.splice(index, 1);
            state = "wrongSpot";
          }
          result.input.push({ key: input[i], state: state });
        }
      }
    }

    if (!result.error) {
      if (!result.input.find((i) => i.state !== "correctSpot")) {
        result.status = "won";
      } else if (numberOfGuesses === ROWS - 1) {
        result.status = "lost";
      }
    }

    return result;
  }

  private createEquation(): void {
    // <<< HERE we call our challenge generation written in Rust
    const challenge: { formula: string; result: number } = generateChallenge();
    this.equation = challenge.formula;
    this.expectedResult = challenge.result;

    // Print equation and result in console. Yes, I know, we are cheating...
    console.log(this.equation);
    console.log(this.expectedResult);
  }
}
