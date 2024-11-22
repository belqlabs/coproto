import { CoprotoBigint } from "./CoprotoBigint";
import { CoprotoBoolean } from "./CoprotoBoolean";
import { CoprotoDouble } from "./CoprotoDouble";
import { CoprotoInteger } from "./CoprotoInteger";
import { CoprotoNull } from "./CoprotoNull";
import { CoprotoString } from "./CoprotoString";

function handleCompositeEncoding(value: object): Uint8Array {
  if(value === null){
    return (new CoprotoNull()).buff;
  }

  throw new Error("Not implemented");
}

export function encode(value: unknown): Uint8Array {
  const typeofValue = typeof value;

  switch (typeofValue) {
    case "number":
      return Number.isInteger(value) ? CoprotoInteger.encode(value as number) : CoprotoDouble.encode(value as number);

    case "string":
      return CoprotoString.encode(value as string);

    case "bigint":
      return CoprotoBigint.encode(value as bigint);

    case "boolean":
      return CoprotoBoolean.encode(value as boolean);

    case "object":
      return handleCompositeEncoding(value as object);

    default:
      throw new Error(`Coproto cant encode ${typeofValue}`);
  }
}
