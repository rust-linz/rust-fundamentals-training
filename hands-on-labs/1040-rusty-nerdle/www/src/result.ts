export type State = 'inProgress' | 'won' | 'lost';
export type CharState = 'correctSpot' | 'wrongSpot' | 'notInSolution';

export class Result {
  error = '';
  status: State = 'inProgress';
  input: {
    key: string;
    state: CharState;
  }[] = [];
}
