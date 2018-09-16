use jwk::data::*;
use jwk::signature::*;


/// KeyProvider is the main trait able to provide the key corresponding to a given specification

pub trait KeyProvider<E> where E: Capability {
    fn key(&self) -> Option<Signature<E>>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Verification> for JSONWebKey<RSAPublicSpecification> {
    fn key(&self) -> Option<Signature<Verification>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Verification> for JSONWebKey<ECPublicSpecification> {
    fn key(&self) -> Option<Signature<Verification>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<RSAPrivateSpecification> {
    fn key(&self) -> Option<Signature<Encryption>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<ECPrivateSpecification> {
    fn key(&self) -> Option<Signature<Encryption>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
