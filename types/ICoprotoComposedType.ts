import { CannonicalCompositeTypeNames } from "./CannonicalCompositeTypeNames";
import { CoprotoCompositeType } from "./CoprotoCompositeType";

export interface ICoprotoCompositeType<Name extends typeof CannonicalCompositeTypeNames[number]>{
  length: number,
  valueOf: CoprotoCompositeType<Name>,
  buff: Uint8Array
}
