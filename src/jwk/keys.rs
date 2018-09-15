use jwk::data::*;
use jwk::signature::*;
use std::error::Error;


/// KeyProvider is the main trait able to provide the key corresponding to a given specification

pub trait KeyProvider<E> {
    fn key(&self) -> Result<Signature<E>, Error>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Verification> for JSONWebKey<RSAPublicSpecification> {
    fn key(&self) -> Result<Signature<Verification>, Error> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Verification> for JSONWebKey<ECPublicSpecification> {
    fn key(&self) -> Result<Signature<Verification>, Error> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<RSAPrivateSpecification> {
    fn key(&self) -> Result<Signature<Encryption>, Error> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<ECPrivateSpecification> {
    fn key(&self) -> Result<Signature<Encryption>, Error> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
