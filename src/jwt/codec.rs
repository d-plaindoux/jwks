extern crate serde_json;

use core::codec::Codec;
use jwt::data::JSONWebToken;

#[derive(Serialize, Deserialize)]
pub struct JSONWebToken4Codec {
    #[serde(rename = "iss")]
    #[serde(default)]
    pub issuer: Option<String>,

    #[serde(rename = "sub")]
    #[serde(default)]
    pub subject: Option<String>,

    #[serde(rename = "aud")]
    #[serde(default)]
    pub audience: Option<Vec<String>>,

    #[serde(rename = "exp")]
    #[serde(default)]
    pub expiration_time: Option<f64>,

    #[serde(rename = "nbf")]
    #[serde(default)]
    pub not_before: Option<f64>,

    #[serde(rename = "iat")]
    #[serde(default)]
    pub issued_at: Option<f64>,

    #[serde(rename = "jit")]
    #[serde(default)]
    pub identifier: Option<String>,
}

// -------------------------------------------------------------------------------------------------

impl Codec<JSONWebToken4Codec> for JSONWebToken4Codec {
    fn encode(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn decode(value: & String) -> Option<JSONWebToken4Codec> {
        serde_json::from_str(value.as_str()).ok()
    }
}

// -------------------------------------------------------------------------------------------------

impl Codec<JSONWebToken> for JSONWebToken {
    fn encode(&self) -> Option<String> {
        let copy = self.clone();

        JSONWebToken4Codec {
            issuer: copy.issuer,
            subject: copy.subject,
            audience: copy.audience,
            expiration_time: copy.expiration_time,
            not_before: copy.not_before,
            issued_at: copy.issued_at,
            identifier: copy.identifier,
        }.encode()
    }

    fn decode(value: &String) -> Option<JSONWebToken> {
        JSONWebToken4Codec::decode(value).map(|v| JSONWebToken {
            issuer: v.issuer,
            subject: v.subject,
            audience: v.audience,
            expiration_time: v.expiration_time,
            not_before: v.not_before,
            issued_at: v.issued_at,
            identifier: v.identifier,
        })
    }
}
