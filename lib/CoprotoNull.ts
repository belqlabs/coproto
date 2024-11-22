import { Delimiters } from "../types/Delimiters";
import { ICoprotoPrimitiveType } from "../types/ICoprotoPrimitiveType";
import { joinParts } from "./utils";

export class CoprotoNull implements ICoprotoPrimitiveType<null>{
  length: undefined;

  modifier?: string | undefined;

  valueOf: null;

  buff: Uint8Array;

  static firstByte = '-';

  static firstByteCharCode = 0x2d;

  static cannonicalType = "null";

  constructor(){
    this.valueOf = null;
    this.buff = joinParts(0x2d, Delimiters.StartRecord, Delimiters.EndRecord, Delimiters.BufferEnd);
  }
}
