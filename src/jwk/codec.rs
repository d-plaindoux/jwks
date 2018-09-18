/**
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at
  http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

extern crate serde_json;

use core::codec::Codec;
use jwk::data::Algorithm;
use jwk::data::JSONWebKey;
use jwk::data::KeyOperation;
use jwk::data::KeyType;
use jwk::data::KeyUse;
use jwk::data::PublicSpecification;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct JSONWebKey4Codec {
    #[serde(rename = "kty")]
    pub key_type: String,

    #[serde(rename = "use")]
    #[serde(default)]
    pub key_use: Option<String>,

    #[serde(rename = "key_ops")]
    #[serde(default)]
    pub key_operation: Option<String>,

    #[serde(rename = "kid")]
    #[serde(default)]
    pub key_id: Option<String>,

    #[serde(rename = "alg")]
    #[serde(default)]
    pub algorithm: Option<String>,

    #[serde(rename = "x5u")]
    #[serde(default)]
    pub x509_url: Option<String>,

    #[serde(rename = "x5uc")]
    #[serde(default)]
    pub x509_chain: Option<Vec<String>>,

    #[serde(rename = "x5t")]
    #[serde(default)]
    pub x509_s1_thumb_print: Option<String>,

    #[serde(rename = "x5t#S256")]
    #[serde(default)]
    pub x509_s256_thumb_print: Option<String>,

    // Specific attribute set for PUBLIC RSA JKW

    #[serde(rename = "n")]
    #[serde(default)]
    pub modulus: Option<String>,

    #[serde(rename = "e")]
    #[serde(default)]
    pub exponent: Option<String>,

    // Specific attribute set for PUBLIC EC JKW

    #[serde(rename = "x")]
    #[serde(default)]
    pub x: Option<String>,

    #[serde(rename = "y")]
    #[serde(default)]
    pub y: Option<String>,

    #[serde(rename = "curve")]
    #[serde(default)]
    pub curve: Option<String>,
}

pub struct ParseError(String);

// -------------------------------------------------------------------------------------------------

impl FromStr for KeyType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<KeyType, Self::Err> {
        match s {
            "EC" => Ok(KeyType::EC),
            "RSA" => Ok(KeyType::RSA),
            "OCTET" => Ok(KeyType::OCTET),
            _ => Err(ParseError("KeyType not found".to_string()))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FromStr for KeyUse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<KeyUse, Self::Err> {
        match s {
            "Sig" => Ok(KeyUse::Sig),
            "Enc" => Ok(KeyUse::Enc),
            _ => Ok(KeyUse::OtherKeyUse(s.to_string()))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FromStr for KeyOperation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<KeyOperation, Self::Err> {
        match s {
            "Sign" => Ok(KeyOperation::Sign),
            "Verify" => Ok(KeyOperation::Verify),
            "Encrypt" => Ok(KeyOperation::Encrypt),
            "Decrypt" => Ok(KeyOperation::Decrypt),
            "WrapKey" => Ok(KeyOperation::WrapKey),
            "UnwrapKey" => Ok(KeyOperation::UnwrapKey),
            "DeriveKeys" => Ok(KeyOperation::DeriveKeys),
            "DeriveBytes" => Ok(KeyOperation::DeriveBytes),
            _ => Ok(KeyOperation::OtherKeyOperation(s.to_string())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FromStr for Algorithm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Algorithm, Self::Err> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            "PS256" => Ok(Algorithm::PS256),
            "PS512" => Ok(Algorithm::PS512),
            _ => Err(ParseError("Algorithm not found".to_string()))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Codec<JSONWebKey4Codec> for JSONWebKey4Codec {
    fn encode(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn decode(value: &String) -> Option<JSONWebKey4Codec> {
        serde_json::from_str(value.as_str()).ok()
    }
}

// -------------------------------------------------------------------------------------------------

fn get_specification(v: &JSONWebKey4Codec) -> Option<PublicSpecification> {
    if v.modulus.is_some() && v.exponent.is_some() {
        return Some(PublicSpecification::RSA {
            modulus: v.modulus.clone().unwrap(),
            exponent: v.exponent.clone().unwrap(),
        });
    }

    if v.x.is_some() && v.y.is_some() && v.curve.is_some() {
        return Some(PublicSpecification::EC {
            x: v.x.clone().unwrap(),
            y: v.y.clone().unwrap(),
            curve: v.curve.clone().unwrap(),
        });
    }

    None
}

impl Codec<JSONWebKey<PublicSpecification>> for JSONWebKey<PublicSpecification> {
    fn encode(&self) -> Option<String> {
        unimplemented!()
    }

    fn decode(value: &String) -> Option<JSONWebKey<PublicSpecification>> {
        JSONWebKey4Codec::decode(value).filter(|v| KeyType::from_str(v.key_type.as_str()).is_ok())
            .and_then(|v| {
                get_specification(&v).map(|k| (v, k))
            })
            .map(|(v, k)|
                JSONWebKey {
                    key_type: KeyType::from_str(v.key_type.as_str()).ok().unwrap(),
                    key_use: v.key_use.and_then(|v| KeyUse::from_str(v.as_str()).ok()),
                    key_operation: v.key_operation.and_then(|v| KeyOperation::from_str(v.as_str()).ok()),
                    key_id: v.key_id,
                    algorithm: v.algorithm.and_then(|v| Algorithm::from_str(v.as_str()).ok()),
                    x509_url: v.x509_url,
                    x509_chain: v.x509_chain,
                    x509_s1_thumb_print: v.x509_s1_thumb_print,
                    x509_s256_thumb_print: v.x509_s256_thumb_print,
                    key_specification: k,
                }
            )
    }
}
