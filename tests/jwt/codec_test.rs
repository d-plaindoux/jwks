use jwtks::core::codec::*;
use jwtks::jwt::codec::*;

#[test]
fn it_decode_a_jwt() {
    let data: &str = r#"
        {
          "sub": "1234567890",
          "name": "John Doe",
          "iat": 1516239022
        }
    "#;

    let jwt_or_none = JSONWebToken4Codec::decode(data);

    assert_eq!(true, jwt_or_none.is_some());
}
