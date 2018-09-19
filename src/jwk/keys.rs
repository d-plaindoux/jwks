use jwk::data::JSONWebKey;
use jwk::data::KeySpecification;
use jwk::data::PrivateSpecification;
use jwk::data::PublicSpecification;

pub trait KeySignature where Self: KeySpecification {
    fn verify_signature(s: &String) -> bool;
}

pub trait KeyEncryption where Self: KeySpecification {
    fn encrypt(s: &String) -> String;
}

// -------------------------------------------------------------------------------------------------

impl KeySignature for PublicSpecification {
    fn verify_signature(s: &String) -> bool {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyEncryption for PrivateSpecification {
    fn encrypt(s: &String) -> String {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
