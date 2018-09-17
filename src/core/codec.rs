pub trait Codec<E> {
    fn encode(&self) -> &str;
    fn decode(value: &str) -> Option<E>;
}