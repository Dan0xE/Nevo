export function shiftArrary<T>(arr: Array<T>, count: number): Array<T> {
  for (let i = 0; i < count; i++) {
    arr.shift();
  }
  return arr;
}
