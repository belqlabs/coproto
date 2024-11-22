import { CoprotoPrimitiveType } from "../types/CoprotoPrimitiveType";
import { ICoprotoCompositeType } from "../types/ICoprotoComposedType";

export class CoprotoNamedValue implements ICoprotoCompositeType<"NamedV"> {
    length: number;
    valueOf: [name: string, value: CoprotoPrimitiveType];
    buff: Uint8Array;

}
