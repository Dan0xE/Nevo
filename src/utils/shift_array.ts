export function shiftArrary(arr: string[], count: number): string[] {
  for (let i = 0; i < count; i++) {
    arr.shift();
  }
  return arr;
}
