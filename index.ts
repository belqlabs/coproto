import { CoprotoArray } from "./lib/CoprotoArray";
import { CoprotoBigint } from "./lib/CoprotoBigint";
import { CoprotoBoolean } from "./lib/CoprotoBoolean";
import { CoprotoCommand } from "./lib/CoprotoCommand";
import { CoprotoDouble } from "./lib/CoprotoDouble";
import { CoprotoInteger } from "./lib/CoprotoInteger";
import { CoprotoNull } from "./lib/CoprotoNull";
import { CoprotoString } from "./lib/CoprotoString";

function testInteger(){
  const integer = new CoprotoInteger(5000);

  const static_encoded = CoprotoInteger.encode(-10000);

  const static_decoded = CoprotoInteger.decode(static_encoded);

  const encoded = integer;
  const decoded = new CoprotoInteger(encoded.buff);

  console.log({
    static_encoded,
    static_decoded,
    encoded,
    decoded
  })
}

function testString(){
  const str = new CoprotoString("Hello World!");

  const static_encoded = CoprotoString.encode("Hello Coproto");

  const static_decoded = CoprotoString.decode(static_encoded);

  const encoded = str;
  const decoded = new CoprotoString(encoded.buff);

  console.log({
    static_encoded,
    static_decoded,
    encoded,
    decoded
  })
}

function testDouble(){
  const double = new CoprotoDouble(0.001);

  const static_encoded = CoprotoDouble.encode(-10.09);

  const static_decoded = CoprotoDouble.decode(static_encoded);

  const encoded = double;

  const decoded = new CoprotoDouble(encoded.buff);

  console.log({
    static_encoded,
    static_decoded,
    encoded,
    decoded
  })
}

function testBoolean(){
  const double = new CoprotoBoolean(true);

  const static_encoded = CoprotoBoolean.encode(false);

  const static_decoded = CoprotoBoolean.decode(static_encoded);

  const encoded = double;

  const decoded = new CoprotoBoolean(encoded.buff);

  console.log({
    static_encoded,
    static_decoded,
    encoded,
    decoded
  })
}

function testBigint(){
  const bigint = new CoprotoBigint(BigInt(123456789101112));

  const static_encoded = CoprotoBigint.encode(BigInt(-123456789101112));

  const static_decoded = CoprotoBigint.decode(static_encoded);

  const encoded = bigint;
  const decoded = new CoprotoBigint(encoded.buff);

  console.log({
    static_encoded,
    static_decoded,
    encoded,
    decoded
  })
}

function testNull(){
  const nullable = new CoprotoNull();
  console.log({
    nullable
  })
}

function testArray(){
  const encoded = new CoprotoArray([1,2, "happy day"]);

  const decoded = new CoprotoArray(encoded.buff);

  const static_encode = CoprotoArray.encode([true, null, 1.053, "5"]);

  const static_decode = CoprotoArray.decode(static_encode);

  console.log({
    encoded,
    decoded,
    static_encode,
    static_decode
  })
}

function testCommand(){
  const encode = new CoprotoCommand(['START', ['server', 1]]);
  
  const decode = new CoprotoCommand(encode.buff);

  const static_encode = CoprotoCommand.encode(...['STOP', ['hiii', false]]);

  const static_decode = CoprotoCommand.decode(static_encode);
  //
  console.log({
    encode: encode.valueOf,
    decode: decode.valueOf,
    static_encode,
    static_decode
  })
}
//testInteger()
//testString()
//testDouble()
//testBoolean()
//testBigint()
//testNull()
//testArray()

testCommand()
