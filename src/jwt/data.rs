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

pub struct JsonWebToken {
    pub issuer: String,                                         // iss
    pub subject: String,                                        // sub
    pub audience: Vec<String>,                                     // aud
    pub expiration_time: String,                                // exp
    pub not_before: String,                                     // nbf
    pub issued_at: String,                                      // iat
    pub identifier: String                                      // jti
}

