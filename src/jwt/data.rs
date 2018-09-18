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

