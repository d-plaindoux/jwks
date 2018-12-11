use crate::jwk::data::KeySpecification;
use crate::jwk::data::PrivateSpecification;
use crate::jwk::data::PublicSpecification;

pub trait KeySignature where Self: KeySpecification {
    fn verify_signature(&self, s: &String) -> bool;
}

pub trait KeyEncryption where Self: KeySpecification {
    fn encrypt(&self, s: &String) -> String;
}

// -------------------------------------------------------------------------------------------------

impl KeySignature for PublicSpecification {
    fn verify_signature(&self, s: &String) -> bool {
        use openssl::bn::BigNum;
        use openssl::rsa::{Rsa, Padding};
        use openssl::pkey::Public;

        match self {
            PublicSpecification::RSA { modulus, exponent } => {
                let modulus = BigNum::from_dec_str(modulus);
                let exponent = BigNum::from_dec_str(exponent);
                let rsa_key = Rsa::<Public>::from_public_components(modulus.unwrap(), exponent.unwrap()).unwrap();
                let input: &[u8] = s.as_ref();
                let output: &mut [u8] = &mut [];
                let _ = rsa_key.public_encrypt(input, output, Padding::PKCS1).unwrap();

                input.eq(output)
            }
            _ => false,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyEncryption for PrivateSpecification {
    fn encrypt(&self, _s: &String) -> String {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------
