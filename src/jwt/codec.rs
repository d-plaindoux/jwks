extern crate serde_json;

use core::codec::Codec;
use jwt::data::JSONWebToken;

#[derive(Serialize, Deserialize)]
pub struct JSONWebToken4Codec {
    pub issuer: String,                                         // iss
    pub subject: String,                                        // sub
    pub audience: Vec<String>,                                  // aud
    pub expiration_time: f64,                                   // exp
    pub not_before: f64,                                        // nbf
    pub issued_at: f64,                                         // iat
    pub identifier: String                                      // jti
}

impl Codec<JSONWebToken> for JSONWebToken4Codec {
    fn encode(&self) -> & str {
        unimplemented!()
    }

    fn decode(value: & str) -> JSONWebToken {
        let result : JSONWebToken4Codec = serde_json::from_str(value).unwrap();

        return JSONWebToken{
            issuer: result.issuer,
            subject: result.subject,
            audience: result.audience,
            expiration_time: result.expiration_time,
            not_before: result.not_before,
            issued_at: result.issued_at,
            identifier: result.identifier
        }
    }
}