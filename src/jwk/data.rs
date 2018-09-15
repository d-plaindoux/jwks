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

/**
   Source: https://tools.ietf.org/html/rfc7517

   The "kty" (key type) parameter identifies the cryptographic algorithm
   family used with the key, such as "RSA" or "EC".  "kty" values should
   either be registered in the IANA "JSON Web Key Types" registry
   established by [JWA] or be a value that contains a Collision-
   Resistant Name.  The "kty" value is a case-sensitive string.  This
   member MUST be present in a JWK.

   Source: https://tools.ietf.org/html/rfc7518

   The table below is the set of "kty" (key type) parameter values that
   are defined by this specification for use in JWKs.

   +-------------+--------------------------------+--------------------+
   | "kty" Param | Key Type                       | Implementation     |
   | Value       |                                | Requirements       |
   +-------------+--------------------------------+--------------------+
   | EC          | Elliptic Curve [DSS]           | Recommended+       |
   | RSA         | RSA [RFC3447]                  | Required           |
   | oct         | Octet sequence (used to        | Required           |
   |             | represent symmetric keys)      |                    |
   +-------------+--------------------------------+--------------------+
*/

pub enum KeyType {
    EC,
    RSA,
    OCTET
}

/**
   Source: https://tools.ietf.org/html/rfc7517

   The "use" (public key use) parameter identifies the intended use of
   the public key.  The "use" parameter is employed to indicate whether
   a public key is used for encrypting data or verifying the signature
   on data.

   Values defined by this specification are:

   o  "sig" (signature)
   o  "enc" (encryption)

   Other values MAY be used.  The "use" value is a case-sensitive
   string.  Use of the "use" member is OPTIONAL, unless the application
   requires its presence.
*/

pub enum KeyUse {
    Sig,
    Enc,
    OtherKeyUse(String)
}

/**
   Source: https://tools.ietf.org/html/rfc7517

   The "key_ops" (key operations) parameter identifies the operation(s)
   for which the key is intended to be used.  The "key_ops" parameter is
   intended for use cases in which public, private, or symmetric keys
   may be present.

   Its value is an array of key operation values.  Values defined by
   this specification are:

   o  "sign" (compute digital signature or MAC)
   o  "verify" (verify digital signature or MAC)
   o  "encrypt" (encrypt content)
   o  "decrypt" (decrypt content and validate decryption, if applicable)
   o  "wrapKey" (encrypt key)
   o  "unwrapKey" (decrypt key and validate decryption, if applicable)
   o  "deriveKey" (derive key)
   o  "deriveBits" (derive bits not to be used as a key)

   Other values MAY be used.  The key operation values are case-
   sensitive strings.  Duplicate key operation values MUST NOT be
   present in the array.  Use of the "key_ops" member is OPTIONAL,
   unless the application requires its presence.
*/

pub enum KeyOperation {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    WrapKey,
    UnwrapKey,
    DeriveKeys,
    DeriveBytes,
    OtherKeyOperation(String)
}

/**
   Source: https://tools.ietf.org/html/rfc7517

   The "alg" (algorithm) parameter identifies the algorithm intended for
   use with the key.  The values used should either be registered in the
   IANA "JSON Web Signature and Encryption Algorithms" registry
   established by [JWA] or be a value that contains a Collision-
   Resistant Name.  The "alg" value is a case-sensitive ASCII string.
   Use of this member is OPTIONAL.

   Source: https://tools.ietf.org/html/rfc7518

   The table below is the set of "alg" (algorithm) Header Parameter
   values defined by this specification for use with JWS, each of which
   is explained in more detail in the following sections:

   +--------------+-------------------------------+--------------------+
   | "alg" Param  | Digital Signature or MAC      | Implementation     |
   | Value        | Algorithm                     | Requirements       |
   +--------------+-------------------------------+--------------------+
   | HS256        | HMAC using SHA-256            | Required           |
   | HS384        | HMAC using SHA-384            | Optional           |
   | HS512        | HMAC using SHA-512            | Optional           |
   | RS256        | RSASSA-PKCS1-v1_5 using       | Recommended        |
   |              | SHA-256                       |                    |
   | RS384        | RSASSA-PKCS1-v1_5 using       | Optional           |
   |              | SHA-384                       |                    |
   | RS512        | RSASSA-PKCS1-v1_5 using       | Optional           |
   |              | SHA-512                       |                    |
   | ES256        | ECDSA using P-256 and SHA-256 | Recommended+       |
   | ES384        | ECDSA using P-384 and SHA-384 | Optional           |
   | ES512        | ECDSA using P-521 and SHA-512 | Optional           |
   | PS256        | RSASSA-PSS using SHA-256 and  | Optional           |
   |              | MGF1 with SHA-256             |                    |
   | PS384        | RSASSA-PSS using SHA-384 and  | Optional           |
   |              | MGF1 with SHA-384             |                    |
   | PS512        | RSASSA-PSS using SHA-512 and  | Optional           |
   |              | MGF1 with SHA-512             |                    |
   | none         | No digital signature or MAC   | Optional           |
   |              | performed                     |                    |
   +--------------+-------------------------------+--------------------+
*/

pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
    PS256,
    PS512
}

/**
   Source: https://tools.ietf.org/html/rfc7517

   A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data
   structure that represents a cryptographic key.  This specification
   also defines a JWK Set JSON data structure that represents a set of
   JWKs.  Cryptographic algorithms and identifiers for use with this
   specification are described in the separate JSON Web Algorithms (JWA)
   specification and IANA registries established by that specification.
*/

pub struct JSONWebKey<E> {
    pub key_type : KeyType,                                     // kty
    pub key_use : Option<KeyUse>,                               // use
    pub key_operation : Option<KeyOperation>,                   // key_ops
    pub key_id : Option<String>,                                // kid
    pub x509_url: String,                                       // x5u
    pub x509_chain: Vec<String>,                                // x5c
    pub x509_s1_thumb_print: String,                            // x5t SHA-1
    pub x509_s256_thumb_print: String,                          // x5t SHA-256
    pub key_specification: E                                    // key specific definition
}

/**
    Additional element used for key externalisation purpose.
*/

pub struct RSAPublicSpecification {
    pub modulus: String,
    pub exponent: String
}

pub struct RSAPrivateSpecification {
    pub modulus: String,
    pub public_exponent: Option<String>,
    pub private_exponent: String
}

pub struct ECPublicSpecification {
    pub x: String,
    pub y: String,
    pub curve: String
}

pub struct ECPrivateSpecification {
    pub x: String,
    pub y: String,
    pub curve: String,
    pub private_exponent: String
}
