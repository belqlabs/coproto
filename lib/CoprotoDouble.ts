import { Delimiters } from "../types/Delimiters";
import { ICoprotoPrimitiveType } from "../types/ICoprotoPrimitiveType";
import { getUpTo, joinParts } from "./utils";

export class CoprotoDouble implements ICoprotoPrimitiveType<number> {
  length = undefined;

  modifier?: string | undefined;

  valueOf: number;

  buff: Uint8Array;

  static firstByte: ';' = ';';

  static firstByteCharCode = 0x3b;

  static cannonicalType = "double";

  constructor(
    value: number | Uint8Array
  ){

    if(typeof value === "number"){
      this.modifier = value < 0 ? '-' : '+';

      this.valueOf = value;

      this.buff = CoprotoDouble.encode(value);

      return;
    }

    this.buff = value;

    this.valueOf = CoprotoDouble.decode(value);

    this.modifier = this.valueOf < 0 ? '-' : '+';

    return;
  }

  static encode(value: number): Uint8Array {
    if(Number.isInteger(value)){
      throw new Error(`[Invalid value] Expected: double, Received: ${value}`);
    }

    const signal = (value < 0 ? '-' : '+').charCodeAt(0);

    const digits = value.toString()
      .split('')
      .map(d => d === '.' ? 0x2e : parseInt(d));
      
    return joinParts(this.firstByteCharCode, signal, Delimiters.StartRecord, digits, Delimiters.EndRecord, Delimiters.BufferEnd);
  }

  static decode(buffer: Uint8Array | Buffer): number {
    const buffArray = Array.from(buffer);

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    const signal = buffArray[0] === 0x2b ? '+' : buffArray[0] === 0x2d ? '-' : '+';

    const start = buffArray.indexOf(Delimiters.StartRecord);

    if(start === -1){
      throw new Error("Could not find the start of the double");
    }

    const integerPart = getUpTo(0x2e, buffArray, start);

    const decimalPart = getUpTo(Delimiters.EndRecord, buffArray, integerPart.foundInIndex);

    return parseFloat(signal + integerPart.slice.join('') + '.' + decimalPart.slice.join(''));
  }

  static isValid(value: number | Buffer | Uint8Array): boolean {
      throw new Error("Method not implemented.");
  }
}