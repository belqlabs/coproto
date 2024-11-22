import { Delimiters } from "../types/Delimiters";
import { ICoprotoPrimitiveType } from "../types/ICoprotoPrimitiveType";
import { CoprotoBoolean } from "./CoprotoBoolean";
import { getUpTo, joinParts } from "./utils";

export class CoprotoBigint implements ICoprotoPrimitiveType<bigint>{
  length: undefined;

  modifier?: string | undefined;

  valueOf: bigint;

  buff: Uint8Array;

  constructor(
    value: bigint | Uint8Array
  ){

    if(typeof value === "bigint"){
      this.modifier = value < 0 ? '-' : '+';

      this.valueOf = value;

      this.buff = CoprotoBigint.encode(value);

      return;
    }

    this.buff = value;

    this.valueOf = CoprotoBigint.decode(value);

    this.modifier = this.valueOf < 0 ? '-' : '+';

    return;
  }

  static firstByte = '(';

  static firstByteCharCode = 0x28;

  static cannonicalType = "bigint";

  static encode(value: bigint): Uint8Array {
    const signal = (value < 0 ? '-' : '+').charCodeAt(0);

    const digits = value.toString()
      .split('')
      .map(d => parseInt(d));
      
    return joinParts(this.firstByteCharCode, signal, Delimiters.StartRecord, digits, Delimiters.EndRecord, Delimiters.BufferEnd);
  }

  static decode(buffer: Uint8Array | Buffer): bigint {
    const buffArray = Array.from(buffer);

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    const signal = buffArray[0] === 0x2b ? '+' : buffArray[0] === 0x2d ? '-' : '+';

    const start = buffArray.indexOf(Delimiters.StartRecord);

    if(start === -1){
      throw new Error("Could not find the start of the integer");
    }

    const buffSlice = getUpTo(Delimiters.EndRecord, buffArray, start);

    return BigInt(signal + buffSlice.slice.join(''));
  }

  static isValid(value: number | Buffer | Uint8Array): boolean {
      throw new Error("Method not implemented.");
  }
}