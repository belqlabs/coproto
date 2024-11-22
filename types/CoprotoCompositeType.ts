import { CannonicalCompositeTypeNames } from "./CannonicalCompositeTypeNames";
import { CoprotoPrimitiveType } from "./CoprotoPrimitiveType";

export type CoprotoCompositeType<Name extends typeof CannonicalCompositeTypeNames[number]> = 
Name extends "Array"    ? CoprotoPrimitiveType[]                              :
Name extends "Command"  ? [command: string, args: CoprotoPrimitiveType[]]     :
Name extends "NamedV"   ? [name: string, value: CoprotoPrimitiveType]         :
Name extends "Table"    ? Record<string, CoprotoPrimitiveType>                :
unknown;
