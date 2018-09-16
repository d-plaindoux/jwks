pub trait Capability {}

// -------------------------------------------------------------------------------------------------

pub struct Verification {}
impl Capability for Verification {}

// -------------------------------------------------------------------------------------------------

pub struct Encryption {}
impl Capability for Encryption {}

// -------------------------------------------------------------------------------------------------

pub struct Signature<E> where E: Capability {
    pub key: E
}

