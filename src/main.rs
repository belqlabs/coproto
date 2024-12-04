use commom::{CoprotoType as _, ValueOrBuffer};
use types::primitive::Double;
use types::primitive::Integer;
use types::BigInt;
use types::Boolean;
use types::Null;
use types::String;

#[allow(dead_code)]
pub mod commom;
pub mod types;

fn main() {
    println!("Integer: --------------------------------------------");
    println!("Positive:");
    let positive_encoding = Integer::new(ValueOrBuffer::Value(1000));
    println!("  Encoding:\n  {:?}", positive_encoding);
    let positive_buff = positive_encoding.buff.unwrap();
    let positive_decoding = Integer::new(ValueOrBuffer::Buffer(positive_buff));
    println!("  Decoding:\n  {:?}", positive_decoding);

    println!("Negative:");
    let negative_encoding = Integer::new(ValueOrBuffer::Value(-1000));
    println!("  Encoding:\n  {:?}", negative_encoding);
    let negative_buff = negative_encoding.buff.unwrap();
    let negative_decoding = Integer::new(ValueOrBuffer::Buffer(negative_buff));
    println!("  Decoding:\n  {:?}", negative_decoding);
    println!("-----------------------------------------------------");

    println!("Double: --------------------------------------------");
    println!("Positive:");
    let positive_encoding = Double::new(ValueOrBuffer::Value(123.456));
    println!("  Encoding:\n  {:?}", positive_encoding);
    let positive_buff = positive_encoding.buff.unwrap();
    let positive_decoding = Double::new(ValueOrBuffer::Buffer(positive_buff));
    println!("  Decoding:\n  {:?}", positive_decoding);

    println!("Negative:");
    let negative_encoding = Double::new(ValueOrBuffer::Value(-123.456));
    println!("  Encoding:\n  {:?}", negative_encoding);
    let negative_buff = negative_encoding.buff.unwrap();
    let negative_decoding = Double::new(ValueOrBuffer::Buffer(negative_buff));
    println!("  Decoding:\n  {:?}", negative_decoding);
    println!("-----------------------------------------------------");

    println!("Boolean: --------------------------------------------");
    println!("True:");
    let true_encoding = Boolean::new(ValueOrBuffer::Value(true));
    println!("  Encoding:\n  {:?}", true_encoding);
    let true_buff = true_encoding.buff.unwrap();
    let true_decoding = Boolean::new(ValueOrBuffer::Buffer(true_buff));
    println!("  Decoding:\n  {:?}", true_decoding);

    println!("False:");
    let false_encoding = Boolean::new(ValueOrBuffer::Value(false));
    println!("  Encoding:\n  {:?}", false_encoding);
    let false_buff = false_encoding.buff.unwrap();
    let false_decoding = Boolean::new(ValueOrBuffer::Buffer(false_buff));
    println!("  Decoding:\n  {:?}", false_decoding);
    println!("-----------------------------------------------------");

    println!("Null: --------------------------------------------");
    let null_encoding = Null::new(ValueOrBuffer::Value(None));
    println!("  Encoding:\n  {:?}", null_encoding);
    let null_buff = null_encoding.buff.unwrap();
    let null_decoding = Null::new(ValueOrBuffer::Buffer(null_buff));
    println!("  Decoding:\n  {:?}", null_decoding);
    println!("-----------------------------------------------------");

    println!("String: --------------------------------------------");
    let string_encoding = String::new(ValueOrBuffer::Value("Hello from the main".to_string()));
    println!("  Encoding:\n  {:?}", string_encoding);
    let string_buff = string_encoding.buff.unwrap();
    let string_decoding = String::new(ValueOrBuffer::Buffer(string_buff));
    println!("  Decoding:\n  {:?}", string_decoding);
    println!("-----------------------------------------------------");

    println!("BigInt: --------------------------------------------");
    println!("Positive:");
    let positive_encoding = BigInt::new(ValueOrBuffer::Value(1000));
    println!("  Encoding:\n  {:?}", positive_encoding);
    let positive_buff = positive_encoding.buff.unwrap();
    let positive_decoding = BigInt::new(ValueOrBuffer::Buffer(positive_buff));
    println!("  Decoding:\n  {:?}", positive_decoding);

    println!("Negative:");
    let negative_encoding = BigInt::new(ValueOrBuffer::Value(-1000));
    println!("  Encoding:\n  {:?}", negative_encoding);
    let negative_buff = negative_encoding.buff.unwrap();
    let negative_decoding = BigInt::new(ValueOrBuffer::Buffer(negative_buff));
    println!("  Decoding:\n  {:?}", negative_decoding);
    println!("-----------------------------------------------------");
}
