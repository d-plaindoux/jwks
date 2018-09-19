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
