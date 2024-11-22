import { Delimiters } from "../types/Delimiters";

export function joinParts(...parts: any[]): Uint8Array {
  const arr: number[] = [];

  for(const part of parts){
    const ensuredArray = Array.isArray(part) ? part : [part];
    arr.push(
      ...ensuredArray
    )
  }

  return Uint8Array.from(arr);
}

export function getUpTo(value: number, array: number[], offset: number = 0): {foundInIndex: number, slice: number[]} {
  const endIdx = array.indexOf(value, offset + 1);

  if(endIdx === -1){
    throw new Error(`Could not find up to ${value.toString()}`)
  }

  return {
    foundInIndex: endIdx,
    slice: array.slice(offset + 1, endIdx)
  }
}

export function splitElements(array: number[]): number[][]{
  const values: number[][] = [];

  let acc: number[] = [];

  for(const entry of array){
    if(entry === Delimiters.ValueDelimiter){
      values.push(acc);
      acc = [];
      continue;
    }  

    acc.push(entry);
  }

  return values;
}
