import { CoprotoPrimitiveType } from "../types/CoprotoPrimitiveType";
import { Delimiters } from "../types/Delimiters";
import { ICoprotoCompositeType } from "../types/ICoprotoComposedType";
import { CoprotoArray } from "./CoprotoArray";
import { CoprotoString } from "./CoprotoString";
import { getUpTo, joinParts, splitElements } from "./utils";

export class CoprotoCommand implements ICoprotoCompositeType<"Command">{
  length: number;

  valueOf: [command: string, args: CoprotoPrimitiveType[]];

  buff: Uint8Array;

  constructor(
    value: [command: string, args: CoprotoPrimitiveType[]] | Uint8Array
  ){
    if(Array.isArray(value) && value.length === 2){
      this.length = value[1].length;
      this.valueOf = value;
      this.buff = CoprotoCommand.encode(value[0], value[1]);

      return
    }

    this.valueOf = CoprotoCommand.decode(value);
    this.length = this.valueOf[1].length;
    this.buff = value;
  }

  static firstByte = '$';

  static firstByteCharCode = 0x24;

  static cannonicalType = "Command";

  static encode(command: string, args: CoprotoPrimitiveType[]): Uint8Array {
    const commandBuff = Array.from(CoprotoString.encode(command));

    const argsBuff = Array.from(CoprotoArray.encode(args));

    argsBuff.pop();

    return joinParts(this.firstByteCharCode, commandBuff, Delimiters.ValueDelimiter, argsBuff, Delimiters.BufferEnd);
  }

  static decode(buff: Uint8Array | Buffer): [command: string, args: CoprotoPrimitiveType[]] {
    const buffArray = Array.from(buff); 

    const firstByteCharCode = buffArray.shift();

    if(firstByteCharCode !== this.firstByteCharCode){
      throw new Error(`Invalid first byte\n  Expected: ${this.firstByteCharCode}\n  Found: ${firstByteCharCode}`);
    }

    const commandSplit = getUpTo(Delimiters.ValueDelimiter, buffArray, -1);

    const command = CoprotoString.decode(Uint8Array.from(commandSplit.slice));

    const argsArray = getUpTo(Delimiters.BufferEnd, buffArray, commandSplit.foundInIndex);

    //console.log(argsArray);

    const args = CoprotoArray.decode(Uint8Array.from(argsArray.slice));

    return [command, args];
  }
}
