import { encode } from "./encode";
import { CoprotoPrimitiveType } from "../types/CoprotoPrimitiveType";
import { ICoprotoCompositeType } from "../types/ICoprotoComposedType";
import { joinParts, splitElements } from "./utils";
import { Delimiters } from "../types/Delimiters";
import { decode } from "./decode";
import { CoprotoCompositeType } from "../types/CoprotoCompositeType";
import { CannonicalCompositeTypeNames } from "../types/CannonicalCompositeTypeNames";

export class CoprotoArray implements ICoprotoCompositeType<"Array"> {
  length: number;

  valueOf: CoprotoPrimitiveType[];

  buff: Uint8Array;

  static firstByte = '[';

  static firstByteCharCode = 0x5b;

  static cannonicalType = "Array";

  constructor(
    value: CoprotoPrimitiveType[] | Uint8Array
  ){
    if(Array.isArray(value)){
      this.length = value.length;

      this.valueOf = value;

      this.buff = CoprotoArray.encode(value);

      return;
    }

    this.buff = value;

    this.valueOf = CoprotoArray.decode(value);

    this.length = this.valueOf.length;
  }

  // TODO: Validar para apenas aceitar valores primitivos

  static encode(values: CoprotoPrimitiveType[]): Uint8Array {
    const length = values.length;

    const arr: number[] = [];

    for(const value of values){
      const encoded = Array.from(encode(value));

      encoded.pop();

      arr.push(
        ...joinParts(encoded, Delimiters.ValueDelimiter)
      )
    }

    return joinParts(this.firstByteCharCode, length, arr, Delimiters.BufferEnd);
  }

  static decode(buff: Uint8Array | Buffer): CoprotoPrimitiveType[]{
    const buffArray = Array.from(buff);

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    buffArray.shift();

    const elements = splitElements(buffArray);

    const arr: CoprotoPrimitiveType[] = [];

    for(const element of elements){
      arr.push(decode(Uint8Array.from(element)) as CoprotoPrimitiveType);
    }

    return arr;
  }
}
