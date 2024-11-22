import { Delimiters } from "../types/Delimiters";
import { ICoprotoPrimitiveType } from "../types/ICoprotoPrimitiveType";
import { getUpTo, joinParts } from "./utils";

export class CoprotoBoolean implements ICoprotoPrimitiveType<boolean>{
  length: undefined;

  modifier = undefined;

  valueOf: boolean;

  buff: Uint8Array;

  constructor(
    value: boolean | Uint8Array
  ){

    if(typeof value === "boolean"){
      this.valueOf = value;

      this.buff = CoprotoBoolean.encode(value);

      return;
    }

    this.buff = value;

    this.valueOf = CoprotoBoolean.decode(value);

    return;
  }

  static firstByte: '#' = '#';

  static firstByteCharCode = 0x23;

  static cannonicalType = "boolean";

  static encode(value: boolean): Uint8Array{
    const bit = value ? 1 : 0;

    return joinParts(this.firstByteCharCode, Delimiters.StartRecord, bit, Delimiters.EndRecord, Delimiters.BufferEnd);
  }

  static decode(buffer: Uint8Array | Buffer): boolean {
    const buffArray = Array.from(buffer);

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    const start = buffArray.indexOf(Delimiters.StartRecord);

    if(start === -1){
      throw new Error("Could not find the start of the double");
    }

    const sliced = getUpTo(Delimiters.EndRecord, buffArray, start);

    return sliced.slice[0] === 1 ? true : false;
  }
}
