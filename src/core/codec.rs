pub trait Codec<E> {

    fn encode(&self) -> Option<String>;

    fn decode(value: &String) -> Option<E>;

}