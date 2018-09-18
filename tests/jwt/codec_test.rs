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

use jwtks::core::codec::*;
use jwtks::jwt::data::*;

#[test]
fn it_decode_a_jwt() {
    let data: &str = r#"
        {
          "sub": "1234567890",
          "name": "John Doe",
          "iat": 1516239022
        }
    "#;

    let jwt_or_none = JSONWebToken::decode(&data.to_string());

    assert_eq!(true, jwt_or_none.is_some());
}

#[test]
fn it_encode_a_jwt() {
    let data: &str = r#"{"iss":null,"sub":"1234567890","aud":null,"exp":null,"nbf":null,"iat":1516239022.0,"jit":null}"#;

    let encoded = JSONWebToken::decode(&data.to_string()).unwrap().encode();

    assert_eq!(data.to_string(), encoded.unwrap());
}
