use std::marker::PhantomData;

pub trait Capability {}

// -------------------------------------------------------------------------------------------------

pub struct Signature {}
impl Capability for Signature {}

// -------------------------------------------------------------------------------------------------

pub struct Encryption {}
impl Capability for Encryption {}

// -------------------------------------------------------------------------------------------------

pub struct Key<E> where E: Capability {
    pub capability: PhantomData<E>
}

