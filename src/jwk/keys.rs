use jwk::signature::Capability;
use jwk::signature::Key;
use jwk::data::JSONWebKey;
use jwk::data::PublicSpecification;
use jwk::signature::Signature;
use jwk::data::PrivateSpecification;
use jwk::signature::Encryption;

/// KeyProvider is the main trait able to provide the key corresponding to a given specification

pub trait KeyProvider<E> where E: Capability {
    fn key(&self) -> Option<Key<E>>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Signature> for JSONWebKey<PublicSpecification> {
    fn key(&self) -> Option<Key<Signature>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<PrivateSpecification> {
    fn key(&self) -> Option<Key<Encryption>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
