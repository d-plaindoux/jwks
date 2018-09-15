use openssl::rsa::Rsa;
use openssl::ec::EcKey;
use openssl::pkey::Public;
use openssl::error::ErrorStack;

use jwks::data::{JSONWebKey, RSAPublicSpecification, ECPublicSpecification};

// -------------------------------------------------------------------------------------------------

pub trait KeyProvider<E> {
    fn key(&self) -> Result<E, ErrorStack>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Rsa<Public>> for JSONWebKey<RSAPublicSpecification> {
    fn key(&self) -> Result<Rsa<Public>, ErrorStack> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<EcKey<Public>> for JSONWebKey<ECPublicSpecification> {
    fn key(&self) -> Result<EcKey<Public>, ErrorStack> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
