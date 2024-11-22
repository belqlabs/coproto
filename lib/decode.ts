import { CannonicalCompositeTypeNames } from "../types/CannonicalCompositeTypeNames";
import { CoprotoCompositeType } from "../types/CoprotoCompositeType";
import { CoprotoPrimitiveType } from "../types/CoprotoPrimitiveType";
import { CoprotoArray } from "./CoprotoArray";
import { CoprotoBigint } from "./CoprotoBigint";
import { CoprotoBoolean } from "./CoprotoBoolean";
import { CoprotoDouble } from "./CoprotoDouble";
import { CoprotoInteger } from "./CoprotoInteger";
import { CoprotoNull } from "./CoprotoNull";
import { CoprotoString } from "./CoprotoString";

export function decode(buff: Uint8Array | Buffer):
CoprotoCompositeType<typeof CannonicalCompositeTypeNames[number]> |
CoprotoPrimitiveType {
  const buffArray = Array.from(buff);

  const firstByte = buffArray[0];

  switch (firstByte) {
    case 0x2b:
      return CoprotoString.decode(buff);      

    case 0x3a:
      return CoprotoInteger.decode(buff);

    case 0x3b:
      return CoprotoDouble.decode(buff);

    case 0x23:
      return CoprotoBoolean.decode(buff);

    case 0x28:
      return CoprotoBigint.decode(buff);

    case 0x2d:
      return new CoprotoNull().valueOf;

    case 0x58: 
      return CoprotoArray.decode(buff);

    case 0x24:
    case 0x40:
    case 0x7b:
      throw new Error("Not implemented");

    default:
      throw new Error(`Unkown first byte ${String.fromCharCode(firstByte)}`)
  }
}
