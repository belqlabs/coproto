import { CoprotoPrimitiveType } from "./CoprotoPrimitiveType";

export interface ICoprotoPrimitiveType<T extends CoprotoPrimitiveType> {
  length: T extends string ? number : undefined,
  modifier?: string,
  valueOf: T,
  buff: Uint8Array
}
