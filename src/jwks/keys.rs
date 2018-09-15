use openssl::rsa::Rsa;
use openssl::ec::EcKey;
use openssl::pkey::Public;
use openssl::error::ErrorStack;

use jwks::data::{JSONWebKey, RSAPublicSpecification, ECPublicSpecification};

pub trait KeyProvider<E> {
    fn public_key(&self) -> Result<E, ErrorStack>;
    fn private_key(&self) -> Option<()>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Rsa<Public>> for JSONWebKey<RSAPublicSpecification> {
    fn public_key(&self) -> Result<Rsa<Public>, ErrorStack> {
        unimplemented!()
    }

    fn private_key(&self) -> Option<()> {
        unimplemented!()
    }
}

impl KeyProvider<EcKey<Public>> for JSONWebKey<ECPublicSpecification> {
    fn public_key(&self) -> Result<EcKey<Public>, ErrorStack> {
        unimplemented!()
    }

    fn private_key(&self) -> Option<()> {
        unimplemented!()
    }
}