/**
   Source: https://tools.ietf.org/html/rfc7519

   The following Claim Names are registered in the IANA "JSON Web Token
   Claims" registry established by Section 10.1.  None of the claims
   defined below are intended to be mandatory to use or implement in all
   cases, but rather they provide a starting point for a set of useful,
   interoperable claims.  Applications using JWTs should define which
   specific claims they use and when they are required or optional.  All
   the names are short because a core goal of JWTs is for the
   representation to be compact.
*/

#[derive(Debug, Clone)]
pub struct JSONWebToken {
    pub issuer: Option<String>,                                 // iss
    pub subject: Option<String>,                                // sub
    pub audience: Option<Vec<String>>,                          // aud
    pub expiration_time: Option<f64>,                           // exp
    pub not_before: Option<f64>,                                // nbf
    pub issued_at: Option<f64>,                                 // iat
    pub identifier: Option<String>                              // jti
}

