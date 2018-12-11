use crate::jwk::data::JSONWebKey;
use crate::jwk::data::PrivateSpecification;
use crate::jwk::data::PublicSpecification;

pub struct JSONWebKeySet<'a> {
    pub key_signature: Vec<&'a JSONWebKey<PublicSpecification>>,
    pub key_encryption: Vec<&'a JSONWebKey<PrivateSpecification>>,
}
