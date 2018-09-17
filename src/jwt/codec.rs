extern crate serde_json;

use core::codec::Codec;
use jwt::data::JSONWebToken;

#[derive(Serialize, Deserialize)]
pub struct JSONWebToken4Codec {
    #[serde(default)]
    pub iss: Option<String>,
    #[serde(default)]
    pub sub: Option<String>,
    #[serde(default)]
    pub aud: Option<Vec<String>>,
    #[serde(default)]
    pub exp: Option<f64>,
    #[serde(default)]
    pub nbf: Option<f64>,
    #[serde(default)]
    pub iat: Option<f64>,
    #[serde(default)]
    pub jit: Option<String>,
}

impl Codec<JSONWebToken> for JSONWebToken4Codec {
    fn encode(&self) -> &str {
        unimplemented!()
    }

    fn decode(value: &str) -> Option<JSONWebToken> {
        let result: Option<JSONWebToken4Codec> = serde_json::from_str(value).ok();

        result.map(|v| JSONWebToken {
            issuer: v.iss,
            subject: v.sub,
            audience: v.aud,
            expiration_time: v.exp,
            not_before: v.nbf,
            issued_at: v.iat,
            identifier: v.jit,
        })
    }
}
