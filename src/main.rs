use commom::{CoprotoType as _, ValueOrBuffer};
use types::primitive::Double;
use types::primitive::Integer;
use types::Array;
use types::BigInt;
use types::Boolean;
use types::NamedValue;
use types::Null;
use types::String;
use types::SupportedTypes;

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
    println!("-----------------------------------------------------\n\n");

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
    println!("-----------------------------------------------------\n\n");

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
    println!("-----------------------------------------------------\n\n");

    println!("Null: --------------------------------------------");
    let null_encoding = Null::new(ValueOrBuffer::Value(None));
    println!("  Encoding:\n  {:?}", null_encoding);
    let null_buff = null_encoding.buff.unwrap();
    let null_decoding = Null::new(ValueOrBuffer::Buffer(null_buff));
    println!("  Decoding:\n  {:?}", null_decoding);
    println!("-----------------------------------------------------\n\n");

    println!("String: --------------------------------------------");
    let string_encoding = String::new(ValueOrBuffer::Value("Hello from the main".to_string()));
    println!("  Encoding:\n  {:?}", string_encoding);
    let string_buff = string_encoding.buff.unwrap();
    let string_decoding = String::new(ValueOrBuffer::Buffer(string_buff));
    println!("  Decoding:\n  {:?}", string_decoding);
    println!("-----------------------------------------------------\n\n");

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
    println!("-----------------------------------------------------\n\n");

    println!("Array: --------------------------------------------");
    println!("Filled:");
    let vec_to_test = vec![
        SupportedTypes::BigInt(123456789),
        SupportedTypes::Boolean(false),
        SupportedTypes::Double(1.2345),
        SupportedTypes::Integer(123),
        SupportedTypes::Null(None),
        SupportedTypes::String("Hello, fellow rustacean!".to_string()),
    ];

    let filled_encoding = Array::new(ValueOrBuffer::Value(vec_to_test.clone()));
    println!("  Encoding:\n  {:?}", filled_encoding);
    let filled_buff = filled_encoding.buff.unwrap();
    let filled_decoding = Array::new(ValueOrBuffer::Buffer(filled_buff));
    println!("  Decoding:\n  {:?}", filled_decoding);

    println!("Empty:");
    let empty_encoding = Array::new(ValueOrBuffer::Value(vec![]));
    println!("  Encoding:\n  {:?}", empty_encoding);
    let empty_buff = empty_encoding.buff.unwrap();
    let empty_decoding = Array::new(ValueOrBuffer::Buffer(empty_buff));
    println!("  Decoding:\n  {:?}", empty_decoding);
    println!("-----------------------------------------------------\n\n");

    println!("NamedValue: --------------------------------------------");
    let types_to_test = [
        SupportedTypes::BigInt(123456789),
        SupportedTypes::Boolean(false),
        SupportedTypes::Double(1.2345),
        SupportedTypes::Integer(123),
        SupportedTypes::Null(None),
        SupportedTypes::String("Hello, fellow rustacean!".to_string()),
    ];

    for tp in types_to_test.iter() {
        println!("  Encoding: {}", tp.get_name());
        let encoding = NamedValue::new(ValueOrBuffer::Value((
            tp.get_name().to_string(),
            tp.clone(),
        )));
        println!("  Encoded:\n    {:?}", encoding);

        let buff = encoding.buff.unwrap();

        println!("  Decoding: {}", tp.get_name());

        let decoding = NamedValue::new(ValueOrBuffer::Buffer(buff));
        println!("  Decode:\n    {:?}", decoding);
    }
    println!("-----------------------------------------------------\n\n");

    println!("Command: --------------------------------------------");
    let encoding = crate::types::Command::new(ValueOrBuffer::Value((
        "OK".to_string(),
        vec![
            "Be".to_string(),
            "happy".to_string(),
            ".".to_string(),
            "Its".to_string(),
            "all".to_string(),
            "ours".to_string(),
        ],
    )));
    println!("Encoded:\n{:?}", encoding);
    let buff = encoding.buff.unwrap();

    let decoding = crate::types::Command::new(ValueOrBuffer::Buffer(buff));
    println!("Decoded:\n{:?}", decoding);
    println!("-----------------------------------------------------\n\n");
}
