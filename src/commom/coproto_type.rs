use super::{errors::TypeResult, Uint8Buff};

pub enum ValueOrBuffer<T> {
    Value(T),
    Buffer(Uint8Buff),
}
pub trait CoprotoType<T> {
    fn new(value: ValueOrBuffer<T>) -> Self;
    fn encode(value: T) -> TypeResult<Uint8Buff>;
    fn decode(value: Uint8Buff) -> TypeResult<T>;
}
