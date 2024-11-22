import { Delimiters } from "../types/Delimiters";
import { ICoprotoPrimitiveType } from "../types/ICoprotoPrimitiveType";
import { getUpTo, joinParts } from "./utils";

export class CoprotoString implements ICoprotoPrimitiveType<string>{
  valueOf: string;

  buff: Uint8Array;

  static firstByte = "+";

  static firstByteCharCode = 0x2b;

  modifier = undefined;

  cannonicalType: "string" = "string";

  length: number;

  constructor(
    value: string | Uint8Array
  ){

    if(typeof value === "string"){
      this.valueOf = value;

      this.buff = CoprotoString.encode(value);

      return;
    }

    this.buff = value;

    this.valueOf = CoprotoString.decode(value);

    return;
  }

  static encode(value: string): Uint8Array {
    const length = value.length;
    
    const charArray = Array.from(Buffer.from(value));

    return joinParts(this.firstByteCharCode, length, Delimiters.StartRecord, charArray, Delimiters.EndRecord, Delimiters.BufferEnd);
  }

  static decode(buffer: Uint8Array | Buffer): string {
    const buffArray = Array.from(buffer);

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    const length = buffArray.shift();

    if(!length){
      throw new Error(`Invalid length byte\n  Expected: integer\n  Found: ${length}`);
    }

    const start = buffArray.indexOf(Delimiters.StartRecord);

    if(start === -1){
      throw new Error("Could not find the start of the string");
    }

    const buffSlice = getUpTo(Delimiters.EndRecord, buffArray, start);

    let str = '';

    for(const char of buffSlice.slice){
      str += String.fromCharCode(char);
    }

    return str;
  }

  static isValid(value: string | Uint8Array | Buffer): boolean {
      throw new Error("Method not implemented.");
  }
}
